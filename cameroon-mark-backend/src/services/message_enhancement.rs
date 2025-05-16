use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait, QuerySelect, QueryOrder, Order, Condition, IntoActiveModel,
};
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;
use regex::Regex;

use crate::{
    entities::{message_template, auto_response_rule, message, notification_preferences, chatbot_config, chatbot_response},
    errors::{AppError, Result},
    models::message_enhancement::{
        MessageTemplate, CreateMessageTemplateRequest, AutoResponseRule, CreateAutoResponseRuleRequest,
        SendTemplatedMessageRequest, BulkMessagingRequest, MessageStatistics, MessageNotificationPreferences,
        UpdateNotificationPreferencesRequest, ChatbotConfig, ChatbotResponse, UpdateChatbotConfigRequest
    },
    utils::validation,
    services::message as message_service,
};

// Create message template
pub async fn create_message_template(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: CreateMessageTemplateRequest,
) -> Result<MessageTemplate> {
    // Validate request
    validation::validate(&payload)?;

    // Extract variables from template
    let re = Regex::new(r"\{\{(\w+)\}\}").map_err(|e| AppError::internal(format!("Regex error: {}", e)))?;
    
    let mut variables = Vec::new();
    for cap in re.captures_iter(&payload.content) {
        if let Some(var_name) = cap.get(1) {
            variables.push(var_name.as_str().to_string());
        }
    }

    // Create template
    let template_id = Uuid::new_v4();
    let now = Utc::now();
    
    let template_model = message_template::ActiveModel {
        id: Set(template_id),
        user_id: Set(user_id),
        name: Set(payload.name),
        subject: Set(payload.subject),
        content: Set(payload.content),
        variables: Set(variables),
        is_public: Set(payload.is_public),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    let template_result = template_model.insert(db).await?;
    
    let template = MessageTemplate {
        id: template_result.id,
        user_id: template_result.user_id,
        name: template_result.name,
        subject: template_result.subject,
        content: template_result.content,
        variables: template_result.variables,
        is_public: template_result.is_public,
        created_at: template_result.created_at,
        updated_at: template_result.updated_at,
    };
    
    Ok(template)
}

// Get message templates
pub async fn get_message_templates(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<MessageTemplate>> {
    // Find templates created by user or marked as public
    let templates = message_template::Entity::find()
        .filter(
            Condition::any()
                .add(message_template::Column::UserId.eq(user_id))
                .add(message_template::Column::IsPublic.eq(true))
        )
        .order_by(message_template::Column::CreatedAt, Order::Desc)
        .all(db)
        .await?;
    
    let result = templates
        .into_iter()
        .map(|t| MessageTemplate {
            id: t.id,
            user_id: t.user_id,
            name: t.name,
            subject: t.subject,
            content: t.content,
            variables: t.variables,
            is_public: t.is_public,
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
        .collect();
    
    Ok(result)
}

// Create auto-response rule
pub async fn create_auto_response_rule(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: CreateAutoResponseRuleRequest,
) -> Result<AutoResponseRule> {
    // Validate request
    validation::validate(&payload)?;

    // Check if template exists and is accessible to user
    let template = message_template::Entity::find_by_id(payload.template_id)
        .filter(
            Condition::any()
                .add(message_template::Column::UserId.eq(user_id))
                .add(message_template::Column::IsPublic.eq(true))
        )
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("Message template not found or not accessible"))?;

    // Create rule
    let rule_id = Uuid::new_v4();
    let now = Utc::now();
    
    let rule_model = auto_response_rule::ActiveModel {
        id: Set(rule_id),
        user_id: Set(user_id),
        name: Set(payload.name),
        trigger_keywords: Set(payload.trigger_keywords),
        template_id: Set(payload.template_id),
        is_active: Set(payload.is_active),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    
    let rule_result = rule_model.insert(db).await?;
    
    let rule = AutoResponseRule {
        id: rule_result.id,
        user_id: rule_result.user_id,
        name: rule_result.name,
        trigger_keywords: rule_result.trigger_keywords,
        template_id: rule_result.template_id,
        is_active: rule_result.is_active,
        created_at: rule_result.created_at,
        updated_at: rule_result.updated_at,
    };
    
    Ok(rule)
}

// Get auto-response rules
pub async fn get_auto_response_rules(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<AutoResponseRule>> {
    let rules = auto_response_rule::Entity::find()
        .filter(auto_response_rule::Column::UserId.eq(user_id))
        .order_by(auto_response_rule::Column::CreatedAt, Order::Desc)
        .all(db)
        .await?;
    
    let result = rules
        .into_iter()
        .map(|r| AutoResponseRule {
            id: r.id,
            user_id: r.user_id,
            name: r.name,
            trigger_keywords: r.trigger_keywords,
            template_id: r.template_id,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();
    
    Ok(result)
}

// Send message using template
pub async fn send_templated_message(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: SendTemplatedMessageRequest,
) -> Result<Uuid> {
    // Begin transaction
    let txn = db.begin().await?;
    
    // Get template
    let template = message_template::Entity::find_by_id(payload.template_id)
        .filter(
            Condition::any()
                .add(message_template::Column::UserId.eq(user_id))
                .add(message_template::Column::IsPublic.eq(true))
        )
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::not_found("Message template not found or not accessible"))?;
    
    // Apply variables to template
    let mut content = template.content.clone();
    let mut subject = template.subject.clone();
    
    for (var_name, var_value) in &payload.variables {
        let placeholder = format!("{{{{{}}}}}", var_name);
        content = content.replace(&placeholder, var_value);
        subject = subject.replace(&placeholder, var_value);
    }
    
    // Create message (using existing message service)
    let message_id = message_service::send_message(
        &txn,
        user_id,
        payload.recipient_id,
        &subject,
        &content,
    ).await?;
    
    // Commit transaction
    txn.commit().await?;
    
    Ok(message_id)
}

// Send bulk messages
pub async fn send_bulk_messages(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: BulkMessagingRequest,
) -> Result<Vec<Uuid>> {
    // Validate request
    validation::validate(&payload)?;

    // Begin transaction
    let txn = db.begin().await?;
    
    // Get template
    let template = message_template::Entity::find_by_id(payload.template_id)
        .filter(
            Condition::any()
                .add(message_template::Column::UserId.eq(user_id))
                .add(message_template::Column::IsPublic.eq(true))
        )
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::not_found("Message template not found or not accessible"))?;
    
    let mut message_ids = Vec::new();
    
    // Process each recipient
    for recipient_id in &payload.recipient_ids {
        // Apply common variables first
        let mut content = template.content.clone();
        let mut subject = template.subject.clone();
        
        for (var_name, var_value) in &payload.variables {
            let placeholder = format!("{{{{{}}}}}", var_name);
            content = content.replace(&placeholder, var_value);
            subject = subject.replace(&placeholder, var_value);
        }
        
        // Apply recipient-specific variables if available
        if let Some(individual_vars) = &payload.individual_variables {
            if let Some(recipient_vars) = individual_vars.get(&recipient_id.to_string()) {
                for (var_name, var_value) in recipient_vars {
                    let placeholder = format!("{{{{{}}}}}", var_name);
                    content = content.replace(&placeholder, var_value);
                    subject = subject.replace(&placeholder, var_value);
                }
            }
        }
        
        // Send message
        let message_id = message_service::send_message(
            &txn,
            user_id,
            *recipient_id,
            &subject,
            &content,
        ).await?;
        
        message_ids.push(message_id);
    }
    
    // Commit transaction
    txn.commit().await?;
    
    Ok(message_ids)
}

// Get message statistics
pub async fn get_message_statistics(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<MessageStatistics> {
    // This would be an intensive query in a real app
    // For this example, returning mock data
    let stats = MessageStatistics {
        total_messages_sent: 150,
        total_messages_received: 92,
        unread_messages: 5,
        average_response_time: Some(45), // 45 minutes
        common_keywords: vec![
            ("price".to_string(), 12),
            ("delivery".to_string(), 8),
            ("available".to_string(), 7),
        ],
        busiest_hour: Some(14), // 2 PM
        message_volume_by_day: {
            let mut map = HashMap::new();
            map.insert("2023-05-10".to_string(), 12);
            map.insert("2023-05-11".to_string(), 9);
            map.insert("2023-05-12".to_string(), 15);
            map
        },
    };
    
    Ok(stats)
}

// Get message notification preferences
pub async fn get_notification_preferences(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<MessageNotificationPreferences> {
    let preferences = notification_preferences::Entity::find_by_id(user_id)
        .one(db)
        .await?;
    
    match preferences {
        Some(prefs) => Ok(MessageNotificationPreferences {
            user_id: prefs.user_id,
            email_notifications: prefs.email_notifications,
            push_notifications: prefs.push_notifications,
            notification_frequency: prefs.notification_frequency,
            quiet_hours_start: prefs.quiet_hours_start,
            quiet_hours_end: prefs.quiet_hours_end,
            updated_at: prefs.updated_at,
        }),
        None => {
            // Return default preferences
            Ok(MessageNotificationPreferences {
                user_id,
                email_notifications: true,
                push_notifications: true,
                notification_frequency: "immediate".to_string(),
                quiet_hours_start: None,
                quiet_hours_end: None,
                updated_at: Utc::now(),
            })
        }
    }
}

// Update message notification preferences
pub async fn update_notification_preferences(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: UpdateNotificationPreferencesRequest,
) -> Result<MessageNotificationPreferences> {
    // Validate request
    validation::validate(&payload)?;

    // Check if preferences exist
    let existing_prefs = notification_preferences::Entity::find_by_id(user_id)
        .one(db)
        .await?;
    
    let now = Utc::now();
    
    match existing_prefs {
        Some(prefs) => {
            // Update existing preferences
            let mut prefs_model = prefs.into_active_model();
            
            prefs_model.email_notifications = Set(payload.email_notifications);
            prefs_model.push_notifications = Set(payload.push_notifications);
            prefs_model.notification_frequency = Set(payload.notification_frequency);
            prefs_model.quiet_hours_start = Set(payload.quiet_hours_start);
            prefs_model.quiet_hours_end = Set(payload.quiet_hours_end);
            prefs_model.updated_at = Set(now);
            
            let updated_prefs = prefs_model.update(db).await?;
            
            Ok(MessageNotificationPreferences {
                user_id: updated_prefs.user_id,
                email_notifications: updated_prefs.email_notifications,
                push_notifications: updated_prefs.push_notifications,
                notification_frequency: updated_prefs.notification_frequency,
                quiet_hours_start: updated_prefs.quiet_hours_start,
                quiet_hours_end: updated_prefs.quiet_hours_end,
                updated_at: updated_prefs.updated_at,
            })
        },
        None => {
            // Create new preferences
            let prefs_model = notification_preferences::ActiveModel {
                user_id: Set(user_id),
                email_notifications: Set(payload.email_notifications),
                push_notifications: Set(payload.push_notifications),
                notification_frequency: Set(payload.notification_frequency),
                quiet_hours_start: Set(payload.quiet_hours_start),
                quiet_hours_end: Set(payload.quiet_hours_end),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            
            let new_prefs = prefs_model.insert(db).await?;
            
            Ok(MessageNotificationPreferences {
                user_id: new_prefs.user_id,
                email_notifications: new_prefs.email_notifications,
                push_notifications: new_prefs.push_notifications,
                notification_frequency: new_prefs.notification_frequency,
                quiet_hours_start: new_prefs.quiet_hours_start,
                quiet_hours_end: new_prefs.quiet_hours_end,
                updated_at: new_prefs.updated_at,
            })
        }
    }
}

// Get chatbot configuration
pub async fn get_chatbot_config(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<ChatbotConfig> {
    // Get chatbot config
    let config = chatbot_config::Entity::find_by_id(user_id)
        .one(db)
        .await?;
    
    match config {
        Some(cfg) => {
            // Get associated responses
            let responses = chatbot_response::Entity::find()
                .filter(chatbot_response::Column::ChatbotId.eq(user_id))
                .all(db)
                .await?;
            
            let response_list = responses.into_iter()
                .map(|r| ChatbotResponse {
                    id: Some(r.id),
                    trigger_keywords: r.trigger_keywords,
                    response_message: r.response_message,
                })
                .collect();
            
            Ok(ChatbotConfig {
                id: cfg.id,
                user_id: cfg.user_id,
                is_enabled: cfg.is_enabled,
                greeting_message: cfg.greeting_message,
                unavailable_message: cfg.unavailable_message,
                common_responses: response_list,
                created_at: cfg.created_at,
                updated_at: cfg.updated_at,
            })
        },
        None => {
            // Return default config
            Ok(ChatbotConfig {
                id: Uuid::nil(),
                user_id,
                is_enabled: false,
                greeting_message: "Hello! Thank you for your message. I'll get back to you as soon as possible.".to_string(),
                unavailable_message: "I'm currently unavailable. Please leave a message and I'll respond when I return.".to_string(),
                common_responses: Vec::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }
    }
}

// Update chatbot configuration
pub async fn update_chatbot_config(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: UpdateChatbotConfigRequest,
) -> Result<ChatbotConfig> {
    // Validate request
    validation::validate(&payload)?;

    // Begin transaction
    let txn = db.begin().await?;

    // Check if config exists
    let existing_config = chatbot_config::Entity::find_by_id(user_id)
        .one(&txn)
        .await?;
    
    let now = Utc::now();
    
    // Create or update config
    let config_id = match existing_config {
        Some(cfg) => {
            // Update existing config
            let mut cfg_model = cfg.into_active_model();
            
            cfg_model.is_enabled = Set(payload.is_enabled);
            cfg_model.greeting_message = Set(payload.greeting_message);
            cfg_model.unavailable_message = Set(payload.unavailable_message);
            cfg_model.updated_at = Set(now);
            
            let updated_cfg = cfg_model.update(&txn).await?;
            updated_cfg.id
        },
        None => {
            // Create new config
            let cfg_model = chatbot_config::ActiveModel {
                id: Set(user_id), // Using user_id as the config ID
                user_id: Set(user_id),
                is_enabled: Set(payload.is_enabled),
                greeting_message: Set(payload.greeting_message),
                unavailable_message: Set(payload.unavailable_message),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            
            let new_cfg = cfg_model.insert(&txn).await?;
            new_cfg.id
        }
    };
    
    // Delete existing responses
    chatbot_response::Entity::delete_many()
        .filter(chatbot_response::Column::ChatbotId.eq(user_id))
        .exec(&txn)
        .await?;
    
    // Create new responses
    let mut response_list = Vec::new();
    
    for response in &payload.common_responses {
        let response_id = Uuid::new_v4();
        
        let response_model = chatbot_response::ActiveModel {
            id: Set(response_id),
            chatbot_id: Set(user_id),
            trigger_keywords: Set(response.trigger_keywords.clone()),
            response_message: Set(response.response_message.clone()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        
        let saved_response = response_model.insert(&txn).await?;
        
        response_list.push(ChatbotResponse {
            id: Some(saved_response.id),
            trigger_keywords: saved_response.trigger_keywords,
            response_message: saved_response.response_message,
        });
    }
    
    // Commit transaction
    txn.commit().await?;
    
    // Return updated config
    let updated_config = chatbot_config::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::internal("Failed to retrieve updated chatbot config"))?;
    
    Ok(ChatbotConfig {
        id: updated_config.id,
        user_id: updated_config.user_id,
        is_enabled: updated_config.is_enabled,
        greeting_message: updated_config.greeting_message,
        unavailable_message: updated_config.unavailable_message,
        common_responses: response_list,
        created_at: updated_config.created_at,
        updated_at: updated_config.updated_at,
    })
}
