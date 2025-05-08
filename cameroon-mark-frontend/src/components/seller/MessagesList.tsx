
import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Search } from 'lucide-react';

// Mock data for demonstration
const mockMessages = [
  {
    id: 'MSG-001',
    sender: 'John Doe',
    subject: 'Question about wooden mask',
    message: 'Hello, I was wondering if the wooden mask is handcrafted and what wood is used? Thank you.',
    date: '2023-05-15T10:30:00',
    read: true
  },
  {
    id: 'MSG-002',
    sender: 'Jane Smith',
    subject: 'Coffee beans order',
    message: 'Hi, I would like to know if you can ship the coffee beans internationally? Best regards, Jane',
    date: '2023-05-14T14:22:00',
    read: false
  },
  {
    id: 'MSG-003',
    sender: 'Robert Johnson',
    subject: 'Return request',
    message: 'I received my order but the product was damaged during shipping. Can you help me with the return process?',
    date: '2023-05-13T09:15:00',
    read: false
  }
];

const MessagesList = () => {
  const [messages] = useState(mockMessages);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedMessage, setSelectedMessage] = useState<string | null>(null);
  
  // Filter messages based on search query
  const filteredMessages = messages.filter(message => 
    message.sender.toLowerCase().includes(searchQuery.toLowerCase()) ||
    message.subject.toLowerCase().includes(searchQuery.toLowerCase()) ||
    message.message.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <Card>
      <CardHeader>
        <CardTitle>Customer Messages</CardTitle>
      </CardHeader>
      <CardContent>
        {/* Search Bar */}
        <div className="relative mb-6">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" />
          <Input
            type="text"
            placeholder="Search messages..."
            className="pl-10"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>

        {/* Messages */}
        {filteredMessages.length === 0 ? (
          <div className="text-center py-10">
            <p className="text-xl font-medium">No messages found</p>
            <p className="text-gray-500 mt-2">
              {searchQuery ? 'Try a different search term' : 'Your inbox is empty'}
            </p>
          </div>
        ) : (
          <div className="border rounded-md overflow-hidden">
            <div className="grid grid-cols-1 md:grid-cols-3 h-[500px]">
              {/* Messages List */}
              <div className="border-r border-gray-200 dark:border-gray-700 overflow-y-auto h-full">
                {filteredMessages.map(message => (
                  <div
                    key={message.id}
                    className={`border-b border-gray-200 dark:border-gray-700 cursor-pointer p-4 ${
                      selectedMessage === message.id 
                        ? 'bg-cameroon-green/10'
                        : 'hover:bg-gray-50 dark:hover:bg-gray-800'
                    } ${!message.read ? 'bg-blue-50 dark:bg-blue-900/10' : ''}`}
                    onClick={() => setSelectedMessage(message.id)}
                  >
                    <div className="flex items-start justify-between mb-1">
                      <span className="font-medium truncate">{message.sender}</span>
                      <span className="text-xs text-gray-500 dark:text-gray-400">
                        {new Date(message.date).toLocaleDateString()}
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <p className="text-sm truncate">{message.subject}</p>
                      {!message.read && (
                        <Badge className="bg-blue-100 text-blue-800 border-blue-200">New</Badge>
                      )}
                    </div>
                  </div>
                ))}
              </div>
              
              {/* Message Content */}
              <div className="col-span-2 flex flex-col h-full">
                {selectedMessage ? (
                  <>
                    {(() => {
                      const message = messages.find(m => m.id === selectedMessage);
                      if (!message) return null;
                      
                      return (
                        <div className="flex flex-col h-full">
                          <div className="p-4 border-b border-gray-200 dark:border-gray-700">
                            <h3 className="text-xl font-medium mb-2">{message.subject}</h3>
                            <div className="flex items-center justify-between text-sm text-gray-500 dark:text-gray-400">
                              <span>From: {message.sender}</span>
                              <span>{new Date(message.date).toLocaleString()}</span>
                            </div>
                          </div>
                          <div className="p-4 flex-grow overflow-auto">
                            <p className="whitespace-pre-line">{message.message}</p>
                          </div>
                          <div className="p-4 border-t border-gray-200 dark:border-gray-700 flex">
                            <Button className="bg-cameroon-green hover:bg-cameroon-green/90">
                              Reply
                            </Button>
                            <Button variant="outline" className="ml-2">
                              Mark as {message.read ? 'Unread' : 'Read'}
                            </Button>
                          </div>
                        </div>
                      );
                    })()}
                  </>
                ) : (
                  <div className="flex items-center justify-center h-full text-center p-4">
                    <div>
                      <p className="text-lg font-medium mb-2">Select a message to view</p>
                      <p className="text-gray-500 dark:text-gray-400">
                        Click on a message from the list on the left to view its contents.
                      </p>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
};

export default MessagesList;
