
import { Link } from 'react-router-dom';
import { Facebook, Twitter, Instagram, Mail, Phone, MapPin } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

const Footer = () => {
  return (
    <footer className="bg-gray-50 dark:bg-gray-900 pt-12 pb-6 border-t">
      <div className="container-custom mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          {/* About */}
          <div>
            <div className="flex items-center mb-4">
              <div className="w-8 h-8 bg-cameroon-green rounded-full flex items-center justify-center">
                <span className="text-white font-bold text-lg">CM</span>
              </div>
              <span className="ml-2 text-xl font-bold text-gray-900 dark:text-white">Cameroon Mark</span>
            </div>
            <p className="text-gray-600 dark:text-gray-300 mb-4">
              Connecting Cameroonian artisans, farmers, and small businesses with local and international buyers.
            </p>
            <div className="flex space-x-4">
              <a href="#" className="text-gray-500 dark:text-gray-400 hover:text-cameroon-green dark:hover:text-cameroon-green">
                <Facebook size={20} />
              </a>
              <a href="#" className="text-gray-500 dark:text-gray-400 hover:text-cameroon-green dark:hover:text-cameroon-green">
                <Twitter size={20} />
              </a>
              <a href="#" className="text-gray-500 dark:text-gray-400 hover:text-cameroon-green dark:hover:text-cameroon-green">
                <Instagram size={20} />
              </a>
            </div>
          </div>

          {/* Quick Links */}
          <div>
            <h3 className="text-lg font-semibold mb-4 text-gray-900 dark:text-white">Quick Links</h3>
            <ul className="space-y-2">
              <li>
                <Link to="/marketplace" className="text-gray-600 dark:text-gray-300 hover:text-cameroon-green dark:hover:text-cameroon-green">
                  Browse Products
                </Link>
              </li>
              <li>
                <Link to="/register?role=seller" className="text-gray-600 dark:text-gray-300 hover:text-cameroon-green dark:hover:text-cameroon-green">
                  Become a Seller
                </Link>
              </li>
              <li>
                <Link to="/about" className="text-gray-600 dark:text-gray-300 hover:text-cameroon-green dark:hover:text-cameroon-green">
                  About Us
                </Link>
              </li>
              <li>
                <Link to="/contact" className="text-gray-600 dark:text-gray-300 hover:text-cameroon-green dark:hover:text-cameroon-green">
                  Contact Us
                </Link>
              </li>
              <li>
                <Link to="/faq" className="text-gray-600 dark:text-gray-300 hover:text-cameroon-green dark:hover:text-cameroon-green">
                  FAQs
                </Link>
              </li>
            </ul>
          </div>

          {/* Contact */}
          <div>
            <h3 className="text-lg font-semibold mb-4 text-gray-900 dark:text-white">Contact</h3>
            <ul className="space-y-3">
              <li className="flex items-start">
                <MapPin size={18} className="text-cameroon-green mt-1 mr-2 flex-shrink-0" />
                <span className="text-gray-600 dark:text-gray-300">
                  Bonanjo, Douala, Cameroon
                </span>
              </li>
              <li className="flex items-center">
                <Phone size={18} className="text-cameroon-green mr-2 flex-shrink-0" />
                <span className="text-gray-600 dark:text-gray-300">+237 6XX XXX XXX</span>
              </li>
              <li className="flex items-center">
                <Mail size={18} className="text-cameroon-green mr-2 flex-shrink-0" />
                <a href="mailto:info@cameroonmark.com" className="text-gray-600 dark:text-gray-300 hover:text-cameroon-green dark:hover:text-cameroon-green">
                  info@cameroonmark.com
                </a>
              </li>
            </ul>
          </div>

          {/* Newsletter */}
          <div>
            <h3 className="text-lg font-semibold mb-4 text-gray-900 dark:text-white">Newsletter</h3>
            <p className="text-gray-600 dark:text-gray-300 mb-4">
              Subscribe to our newsletter for updates on new products and features.
            </p>
            <div className="flex space-x-2">
              <Input 
                placeholder="Your email" 
                className="bg-white dark:bg-gray-800" 
              />
              <Button className="bg-cameroon-green hover:bg-cameroon-green/90">
                Subscribe
              </Button>
            </div>
          </div>
        </div>

        {/* Copyright */}
        <div className="mt-10 pt-6 border-t border-gray-200 dark:border-gray-800">
          <div className="flex flex-col md:flex-row justify-between items-center">
            <p className="text-gray-600 dark:text-gray-400 text-sm mb-4 md:mb-0">
              © {new Date().getFullYear()} Cameroon Mark. All rights reserved.
            </p>
            <div className="flex space-x-4 text-sm text-gray-600 dark:text-gray-400">
              <Link to="/privacy" className="hover:text-cameroon-green dark:hover:text-cameroon-green">
                Privacy Policy
              </Link>
              <Link to="/terms" className="hover:text-cameroon-green dark:hover:text-cameroon-green">
                Terms of Service
              </Link>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
