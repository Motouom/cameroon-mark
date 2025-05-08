
import { ShoppingCart, PenTool, TruckIcon, CreditCard } from 'lucide-react';

const HowItWorks = () => {
  const steps = [
    {
      icon: <ShoppingCart className="h-12 w-12 text-cameroon-yellow" />,
      title: 'Browse',
      description: 'Explore our wide range of authentic Cameroonian products, from art to food.',
    },
    {
      icon: <CreditCard className="h-12 w-12 text-cameroon-yellow" />,
      title: 'Purchase',
      description: 'Securely pay using MTN Mobile Money, Orange Money, or other payment methods.',
    },
    {
      icon: <TruckIcon className="h-12 w-12 text-cameroon-yellow" />,
      title: 'Delivery',
      description: 'Get your products delivered directly to your doorstep across Cameroon and internationally.',
    },
    {
      icon: <PenTool className="h-12 w-12 text-cameroon-yellow" />,
      title: 'Sell',
      description: 'Are you an artisan or producer? Register and start selling your products to a global audience.',
    },
  ];

  return (
    <section className="py-16 bg-white dark:bg-gray-950">
      <div className="container-custom mx-auto">
        {/* Section Title */}
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold mb-3 text-gray-900 dark:text-white">How It Works</h2>
          <p className="text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
            Simple steps to connect with authentic Cameroonian products and sellers
          </p>
        </div>
        
        {/* Steps */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          {steps.map((step, index) => (
            <div key={index} className="text-center">
              <div className="bg-cameroon-green/10 dark:bg-cameroon-green/5 w-24 h-24 rounded-full flex items-center justify-center mx-auto mb-4">
                {step.icon}
              </div>
              <h3 className="text-xl font-semibold mb-2 text-gray-900 dark:text-white">{step.title}</h3>
              <p className="text-gray-600 dark:text-gray-300">{step.description}</p>
            </div>
          ))}
        </div>
        
        {/* Connection Line (visible on desktop) */}
        <div className="hidden lg:block relative h-1 mt-16">
          <div className="absolute top-12 left-0 right-0 border-t-2 border-dashed border-cameroon-green/30"></div>
        </div>
      </div>
    </section>
  );
};

export default HowItWorks;
