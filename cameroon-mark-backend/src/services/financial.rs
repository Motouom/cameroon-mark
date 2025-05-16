use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait, QuerySelect, QueryOrder, Order, IntoActiveModel,
};
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use std::collections::HashMap;

use crate::{
    entities::{payment, expense, order, order_item, product},
    errors::{AppError, Result},
    models::financial::{
        Payment, PaymentProcessor, PaymentStatus, ProcessPaymentRequest, PaymentResponse,
        RefundRequest, RevenueReport, CategoryRevenue, ProductRevenue, TaxCalculation,
        TaxItem, CalculateTaxRequest, Expense, CreateExpenseRequest, FinancialStatement,
        IncomeStatement, BalanceSheet, CashFlow, FinancialReportRequest
    },
    utils::validation,
};

// Process payment (MTN Mobile Money, Orange Money, etc.)
pub async fn process_payment(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: ProcessPaymentRequest,
) -> Result<PaymentResponse> {
    // Validate request
    validation::validate(&payload)?;

    // Check if order exists and belongs to user
    let order = order::Entity::find_by_id(payload.order_id)
        .filter(order::Column::UserId.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Order not found"))?;
    
    // Verify payment amount matches order total
    if payload.amount != order.total_amount {
        return Err(AppError::bad_request("Payment amount does not match order total"));
    }
    
    // Begin transaction
    let txn = db.begin().await?;
    
    // Create payment record
    let payment_id = Uuid::new_v4();
    let now = Utc::now();
    
    let payment_model = payment::ActiveModel {
        id: Set(payment_id),
        order_id: Set(payload.order_id),
        user_id: Set(user_id),
        amount: Set(payload.amount),
        currency: Set(payload.currency),
        processor: Set(payload.processor.to_string()),
        processor_payment_id: Set(None),
        status: Set(PaymentStatus::Pending.to_string()),
        payment_method: Set(payload.payment_method),
        metadata: Set(serde_json::to_value(payload.metadata).ok()),
        created_at: Set(now),
        updated_at: Set(now),
        completed_at: Set(None),
        ..Default::default()
    };
    
    let payment_result = payment_model.insert(&txn).await?;
    
    // In a real implementation, this would integrate with the actual payment provider API
    // For now, we'll simulate the payment process
    
    let (payment_status, redirect_url, processor_reference, message) = match payload.processor {
        PaymentProcessor::MtnMobileMoney => {
            if let Some(phone) = &payload.phone_number {
                // Simulate successful payment for specific test phone numbers
                if phone.ends_with("1234") {
                    (
                        PaymentStatus::Completed,
                        None,
                        Some(format!("MTN-{}", Uuid::new_v4())),
                        "Payment completed successfully".to_string()
                    )
                } else {
                    (
                        PaymentStatus::Pending,
                        Some(format!("https://mtn-money.example.com/pay/{}", payment_id)),
                        Some(format!("MTN-PENDING-{}", payment_id)),
                        "Payment initiated. Complete the payment on your mobile phone".to_string()
                    )
                }
            } else {
                (
                    PaymentStatus::Failed,
                    None,
                    None,
                    "Phone number required for Mobile Money payments".to_string()
                )
            }
        },
        PaymentProcessor::OrangeMoney => {
            if let Some(phone) = &payload.phone_number {
                // Simulate payment
                (
                    PaymentStatus::Pending,
                    Some(format!("https://orange-money.example.com/pay/{}", payment_id)),
                    Some(format!("OM-{}", Uuid::new_v4())),
                    "Payment initiated. Complete the payment on your mobile phone".to_string()
                )
            } else {
                (
                    PaymentStatus::Failed,
                    None,
                    None,
                    "Phone number required for Mobile Money payments".to_string()
                )
            }
        },
        _ => (
            PaymentStatus::Processing,
            None,
            Some(format!("PROC-{}", Uuid::new_v4())),
            "Payment is being processed".to_string()
        )
    };
    
    // Update payment status
    let mut payment_update = payment_result.clone().into_active_model();
    payment_update.status = Set(payment_status.to_string());
    payment_update.processor_payment_id = Set(processor_reference.clone());
    
    if payment_status == PaymentStatus::Completed {
        payment_update.completed_at = Set(Some(Utc::now()));
        
        // Update order status
        let mut order_update = order.into_active_model();
        order_update.status = Set("paid".to_string());
        order_update.updated_at = Set(Utc::now());
        
        order_update.update(&txn).await?;
    }
    
    payment_update.update(&txn).await?;
    
    // Commit transaction
    txn.commit().await?;
    
    // Return payment result
    let response = PaymentResponse {
        payment_id,
        status: payment_status,
        redirect_url,
        processor_reference,
        message,
    };
    
    Ok(response)
}

// Process refund
pub async fn process_refund(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: RefundRequest,
) -> Result<PaymentResponse> {
    // Validate request
    validation::validate(&payload)?;

    // Get payment
    let payment_record = payment::Entity::find_by_id(payload.payment_id)
        .filter(payment::Column::UserId.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Payment not found"))?;
    
    // Check if payment is eligible for refund
    if payment_record.status != PaymentStatus::Completed.to_string() {
        return Err(AppError::bad_request("Only completed payments can be refunded"));
    }
    
    // Determine refund amount
    let refund_amount = payload.amount.unwrap_or(payment_record.amount);
    
    if refund_amount > payment_record.amount {
        return Err(AppError::bad_request("Refund amount cannot exceed payment amount"));
    }
    
    // Begin transaction
    let txn = db.begin().await?;
    
    // In a real implementation, this would call the payment provider's API
    // For this example, we'll simulate the refund process
    
    // Update payment status
    let mut payment_update = payment_record.clone().into_active_model();
    
    if refund_amount == payment_record.amount {
        // Full refund
        payment_update.status = Set(PaymentStatus::Refunded.to_string());
    } else {
        // Partial refund
        payment_update.status = Set(PaymentStatus::PartiallyRefunded.to_string());
    }
    
    payment_update.updated_at = Set(Utc::now());
    payment_update.update(&txn).await?;
    
    // Update order status
    let order = order::Entity::find_by_id(payment_record.order_id)
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::internal("Order not found for payment"))?;
    
    let mut order_update = order.into_active_model();
    
    if refund_amount == payment_record.amount {
        order_update.status = Set("refunded".to_string());
    } else {
        order_update.status = Set("partially_refunded".to_string());
    }
    
    order_update.updated_at = Set(Utc::now());
    order_update.update(&txn).await?;
    
    // Create refund record in a real implementation
    
    // Commit transaction
    txn.commit().await?;
    
    // Return response
    let response = PaymentResponse {
        payment_id: payment_record.id,
        status: if refund_amount == payment_record.amount {
            PaymentStatus::Refunded
        } else {
            PaymentStatus::PartiallyRefunded
        },
        redirect_url: None,
        processor_reference: Some(format!("REFUND-{}", Uuid::new_v4())),
        message: format!("Refund of {} {} processed successfully", refund_amount, payment_record.currency),
    };
    
    Ok(response)
}

// Get revenue report
pub async fn get_revenue_report(
    db: &DatabaseConnection,
    user_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<RevenueReport> {
    // In a real implementation, this would query the database for orders, payments, etc.
    // For this example, we'll return mock data
    
    let revenue_report = RevenueReport {
        total_revenue: 50000.0,
        net_revenue: 45000.0,
        processing_fees: 1500.0,
        refunds: 2000.0,
        taxes_collected: 1500.0,
        revenue_by_day: {
            let mut map = HashMap::new();
            map.insert("2023-05-01".to_string(), 2500.0);
            map.insert("2023-05-02".to_string(), 3000.0);
            map.insert("2023-05-03".to_string(), 3500.0);
            map
        },
        revenue_by_payment_method: {
            let mut map = HashMap::new();
            map.insert("MTN Mobile Money".to_string(), 30000.0);
            map.insert("Orange Money".to_string(), 15000.0);
            map.insert("Cash on Delivery".to_string(), 5000.0);
            map
        },
        revenue_by_category: vec![
            CategoryRevenue {
                category_id: Uuid::new_v4(),
                category_name: "Electronics".to_string(),
                revenue: 25000.0,
                percentage: 50.0,
            },
            CategoryRevenue {
                category_id: Uuid::new_v4(),
                category_name: "Clothing".to_string(),
                revenue: 15000.0,
                percentage: 30.0,
            },
            CategoryRevenue {
                category_id: Uuid::new_v4(),
                category_name: "Home Goods".to_string(),
                revenue: 10000.0,
                percentage: 20.0,
            },
        ],
        top_products: vec![
            ProductRevenue {
                product_id: Uuid::new_v4(),
                product_name: "Smartphone".to_string(),
                revenue: 15000.0,
                units_sold: 30,
            },
            ProductRevenue {
                product_id: Uuid::new_v4(),
                product_name: "Laptop".to_string(),
                revenue: 10000.0,
                units_sold: 10,
            },
            ProductRevenue {
                product_id: Uuid::new_v4(),
                product_name: "T-shirt".to_string(),
                revenue: 5000.0,
                units_sold: 100,
            },
        ],
    };
    
    Ok(revenue_report)
}

// Calculate taxes
pub async fn calculate_taxes(
    db: &DatabaseConnection,
    payload: CalculateTaxRequest,
) -> Result<TaxCalculation> {
    // Validate request
    validation::validate(&payload)?;

    // In a real implementation, this would use tax rules based on jurisdiction
    // For this example, we'll simulate a simple tax calculation
    
    // Apply a basic tax rate based on shipping address
    let base_tax_rate = match &payload.shipping_address {
        Some(address) => {
            match address.country.as_str() {
                "Cameroon" => 0.195, // 19.5% VAT
                _ => 0.0,  // No tax for international
            }
        },
        None => 0.195, // Default to Cameroon VAT
    };
    
    let mut tax_items = Vec::new();
    let mut total_tax = 0.0;
    
    // Calculate tax for each item
    for item in &payload.items {
        let item_total = item.unit_price * item.quantity as f64;
        let item_tax = item_total * base_tax_rate;
        total_tax += item_tax;
        
        tax_items.push(TaxItem {
            name: "VAT".to_string(),
            rate: base_tax_rate as f32,
            amount: item_tax,
        });
    }
    
    // Calculate grand total
    let grand_total = payload.subtotal + payload.shipping_cost + total_tax;
    
    let tax_calculation = TaxCalculation {
        order_id: payload.order_id,
        subtotal: payload.subtotal,
        shipping_cost: payload.shipping_cost,
        taxes: tax_items,
        total_tax,
        grand_total,
    };
    
    Ok(tax_calculation)
}

// Track expense
pub async fn track_expense(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: CreateExpenseRequest,
) -> Result<Expense> {
    // Validate request
    validation::validate(&payload)?;

    // Create expense record
    let expense_id = Uuid::new_v4();
    let now = Utc::now();
    
    let expense_model = expense::ActiveModel {
        id: Set(expense_id),
        user_id: Set(user_id),
        amount: Set(payload.amount),
        currency: Set(payload.currency),
        category: Set(payload.category),
        description: Set(payload.description),
        date: Set(payload.date),
        receipt_url: Set(payload.receipt_url),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    let expense_result = expense_model.insert(db).await?;
    
    let expense = Expense {
        id: expense_result.id,
        user_id: expense_result.user_id,
        amount: expense_result.amount,
        currency: expense_result.currency,
        category: expense_result.category,
        description: expense_result.description,
        date: expense_result.date,
        receipt_url: expense_result.receipt_url,
        created_at: expense_result.created_at,
        updated_at: expense_result.updated_at,
    };
    
    Ok(expense)
}

// Get expenses
pub async fn get_expenses(
    db: &DatabaseConnection,
    user_id: Uuid,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    category: Option<String>,
) -> Result<Vec<Expense>> {
    let mut query = expense::Entity::find()
        .filter(expense::Column::UserId.eq(user_id))
        .order_by(expense::Column::Date, Order::Desc);
    
    if let Some(start) = start_date {
        query = query.filter(expense::Column::Date.gte(start));
    }
    
    if let Some(end) = end_date {
        query = query.filter(expense::Column::Date.lte(end));
    }
    
    if let Some(cat) = category {
        query = query.filter(expense::Column::Category.eq(cat));
    }
    
    let expenses = query.all(db).await?;
    
    let result = expenses.into_iter()
        .map(|e| Expense {
            id: e.id,
            user_id: e.user_id,
            amount: e.amount,
            currency: e.currency,
            category: e.category,
            description: e.description,
            date: e.date,
            receipt_url: e.receipt_url,
            created_at: e.created_at,
            updated_at: e.updated_at,
        })
        .collect();
    
    Ok(result)
}

// Generate financial statement
pub async fn generate_financial_statement(
    db: &DatabaseConnection,
    user_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<FinancialStatement> {
    // In a real implementation, this would query various financial records
    // For this example, we'll return mock data
    
    let statement = FinancialStatement {
        period_start: start_date,
        period_end: end_date,
        income_statement: IncomeStatement {
            revenue: 50000.0,
            cost_of_goods_sold: 25000.0,
            gross_profit: 25000.0,
            gross_margin: 50.0,
            operating_expenses: {
                let mut map = HashMap::new();
                map.insert("Rent".to_string(), 2000.0);
                map.insert("Utilities".to_string(), 500.0);
                map.insert("Salaries".to_string(), 10000.0);
                map.insert("Marketing".to_string(), 3000.0);
                map.insert("Other".to_string(), 1500.0);
                map
            },
            total_operating_expenses: 17000.0,
            operating_income: 8000.0,
            other_income: 500.0,
            other_expenses: 300.0,
            net_income: 8200.0,
            net_margin: 16.4,
        },
        balance_sheet: BalanceSheet {
            assets: {
                let mut map = HashMap::new();
                map.insert("Cash".to_string(), 15000.0);
                map.insert("Accounts Receivable".to_string(), 5000.0);
                map.insert("Inventory".to_string(), 20000.0);
                map.insert("Equipment".to_string(), 10000.0);
                map
            },
            total_assets: 50000.0,
            liabilities: {
                let mut map = HashMap::new();
                map.insert("Accounts Payable".to_string(), 8000.0);
                map.insert("Loans".to_string(), 10000.0);
                map
            },
            total_liabilities: 18000.0,
            equity: {
                let mut map = HashMap::new();
                map.insert("Owner's Equity".to_string(), 24000.0);
                map.insert("Retained Earnings".to_string(), 8000.0);
                map
            },
            total_equity: 32000.0,
        },
        cash_flow: CashFlow {
            operating_activities: {
                let mut map = HashMap::new();
                map.insert("Net Income".to_string(), 8200.0);
                map.insert("Depreciation".to_string(), 1000.0);
                map.insert("Changes in Working Capital".to_string(), -2000.0);
                map
            },
            net_cash_from_operating: 7200.0,
            investing_activities: {
                let mut map = HashMap::new();
                map.insert("Purchase of Equipment".to_string(), -3000.0);
                map
            },
            net_cash_from_investing: -3000.0,
            financing_activities: {
                let mut map = HashMap::new();
                map.insert("Loan Payments".to_string(), -1000.0);
                map
            },
            net_cash_from_financing: -1000.0,
            net_change_in_cash: 3200.0,
            beginning_cash_balance: 11800.0,
            ending_cash_balance: 15000.0,
        },
    };
    
    Ok(statement)
}
