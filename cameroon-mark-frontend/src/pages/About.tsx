
import React from 'react';
import Layout from '@/components/layout/Layout';

const About = () => {
  return (
    <Layout>
      <div className="container mx-auto py-16 px-4">
        <div className="max-w-4xl mx-auto">
          <h1 className="text-4xl font-bold mb-8 text-gray-900 dark:text-white">About Cameroon Mark</h1>
          
          <div className="prose prose-lg dark:prose-invert">
            <p>
              Welcome to Cameroon Mark, the premier online marketplace dedicated to authentic
              Cameroonian products. Our platform connects artisans, farmers, and businesses
              from Cameroon with customers worldwide who appreciate unique, high-quality goods.
            </p>
            
            <h2>Our Mission</h2>
            <p>
              Our mission is to promote Cameroonian culture and craftsmanship while creating
              sustainable economic opportunities for local producers. We believe in fair trade
              practices and supporting small businesses that represent the rich cultural heritage
              of Cameroon.
            </p>
            
            <h2>Our Story</h2>
            <p>
              Founded in 2023, Cameroon Mark began as a small initiative to help local artisans
              reach international markets. Today, we've grown into a thriving marketplace featuring
              thousands of products across multiple categories, from traditional crafts to modern goods.
            </p>
            
            <h2>Quality Assurance</h2>
            <p>
              Every product on Cameroon Mark undergoes careful verification to ensure authenticity
              and quality. We work directly with sellers to maintain high standards and provide
              customers with genuine Cameroonian products.
            </p>
            
            <h2>Join Our Community</h2>
            <p>
              Whether you're a buyer looking for unique products or a seller wanting to expand
              your reach, we invite you to join our growing community. Together, we can celebrate
              and share the best of Cameroon with the world.
            </p>
          </div>
        </div>
      </div>
    </Layout>
  );
};

export default About;
