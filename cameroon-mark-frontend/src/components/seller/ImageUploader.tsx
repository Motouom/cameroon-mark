
import { useState, useCallback } from 'react';
import { useDropzone } from 'react-dropzone';
import { Button } from '@/components/ui/button';
import { X, Upload, Image, Trash } from 'lucide-react';
import { useToast } from '@/components/ui/use-toast';

interface ImageUploaderProps {
  images: string[];
  onUpload: (uploadedImageUrls: string[]) => void;
  onRemove: (index: number) => void;
  maxImages?: number;
  maxSizeMB?: number;
}

const ImageUploader = ({
  images,
  onUpload,
  onRemove,
  maxImages = 6,
  maxSizeMB = 5,
}: ImageUploaderProps) => {
  const { toast } = useToast();
  const [isUploading, setIsUploading] = useState(false);
  
  // Convert maxSizeMB to bytes
  const maxSizeBytes = maxSizeMB * 1024 * 1024;

  const onDrop = useCallback(
    async (acceptedFiles: File[]) => {
      // Check if we would exceed maximum number of images
      if (images.length + acceptedFiles.length > maxImages) {
        toast({
          title: "Too many images",
          description: `You can only upload a maximum of ${maxImages} images.`,
          variant: "destructive",
        });
        return;
      }

      // Filter invalid files
      const validFiles = acceptedFiles.filter(file => {
        // Check file type
        if (!file.type.startsWith('image/')) {
          toast({
            title: "Invalid file type",
            description: `${file.name} is not an image file.`,
            variant: "destructive",
          });
          return false;
        }
        
        // Check file size
        if (file.size > maxSizeBytes) {
          toast({
            title: "File too large",
            description: `${file.name} exceeds the ${maxSizeMB}MB limit.`,
            variant: "destructive",
          });
          return false;
        }
        
        return true;
      });
      
      if (validFiles.length === 0) return;
      
      setIsUploading(true);
      
      try {
        // In a real app, this is where you would upload to MinIO/S3
        // For this demo, we'll create object URLs as placeholders
        
        // Simulate API delay
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        // Create preview URLs (in a real app, these would be actual URLs from your storage)
        const uploadedUrls = validFiles.map(file => URL.createObjectURL(file));
        
        onUpload(uploadedUrls);
        
        toast({
          title: "Upload successful",
          description: `Successfully uploaded ${validFiles.length} ${validFiles.length === 1 ? 'image' : 'images'}.`,
        });
      } catch (error) {
        console.error('Error uploading images:', error);
        toast({
          title: "Upload failed",
          description: "There was a problem uploading your images. Please try again.",
          variant: "destructive",
        });
      } finally {
        setIsUploading(false);
      }
    },
    [images.length, maxImages, maxSizeBytes, maxSizeMB, onUpload, toast]
  );

  const { getRootProps, getInputProps, isDragActive, open } = useDropzone({
    onDrop,
    accept: {
      'image/jpeg': [],
      'image/png': [],
      'image/webp': [],
      'image/gif': []
    },
    noClick: true,
    noKeyboard: true,
    maxSize: maxSizeBytes,
  });

  // Render image previews with the first image marked as primary
  const renderImagePreviews = () => {
    return (
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4 mt-4">
        {images.map((imageUrl, index) => (
          <div 
            key={index} 
            className={`group relative aspect-square rounded-md overflow-hidden border ${index === 0 ? 'border-primary border-2' : 'border-gray-200'}`}
          >
            <img
              src={imageUrl}
              alt={`Product image ${index + 1}`}
              className="w-full h-full object-cover"
            />
            
            {index === 0 && (
              <div className="absolute top-0 left-0 bg-primary text-white text-xs px-2 py-1 rounded-br">
                Primary
              </div>
            )}
            
            <div className="absolute inset-0 bg-black bg-opacity-40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
              <Button
                variant="destructive"
                size="sm"
                className="rounded-full"
                onClick={() => onRemove(index)}
              >
                <Trash className="h-4 w-4" />
              </Button>
            </div>
          </div>
        ))}
      </div>
    );
  };

  return (
    <div className="space-y-4">
      {/* Dropzone area */}
      <div
        {...getRootProps()}
        className={`border-2 border-dashed rounded-lg p-6 text-center transition-colors ${
          isDragActive
            ? 'border-primary bg-primary/5'
            : 'border-gray-300 hover:border-primary/50'
        }`}
      >
        <input {...getInputProps()} />
        <div className="flex flex-col items-center justify-center space-y-3">
          <div className="rounded-full bg-primary/10 p-3">
            <Image className="h-6 w-6 text-primary" />
          </div>
          <div>
            <p className="text-sm font-medium">
              {isDragActive ? 'Drop images here' : 'Drag and drop images here'}
            </p>
            <p className="text-xs text-gray-500 mt-1">
              Supports JPG, PNG, WEBP and GIF up to {maxSizeMB}MB
            </p>
          </div>
          <Button 
            type="button" 
            variant="outline" 
            disabled={isUploading || images.length >= maxImages}
            onClick={open}
            className="mt-2"
          >
            {isUploading ? 'Uploading...' : 'Select Files'}
          </Button>
        </div>
      </div>

      {/* Image preview section */}
      {images.length > 0 && renderImagePreviews()}
      
      {/* Upload status */}
      {isUploading && (
        <div className="flex items-center justify-center text-sm text-gray-500">
          <Upload className="animate-bounce h-4 w-4 mr-2" />
          Uploading images...
        </div>
      )}
    </div>
  );
};

export default ImageUploader;
