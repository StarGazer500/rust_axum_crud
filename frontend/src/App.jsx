import { useState } from 'react'
import './App.css'
import { Send, Loader2, CheckCircle, XCircle } from 'lucide-react'

function App() {
  const [formData, setFormData] = useState({
    email: '',
    password: '',
  });
  const [isLoading, setIsLoading] = useState(false);
  const [response, setResponse] = useState(null);
  const [error, setError] = useState(null);

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
  };

  const handlePostRequest = async () => {
    setIsLoading(true);
    setResponse(null);
    setError(null);

    try {
      const response = await fetch('http://localhost:3000/crud/save_credentials', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(formData)
      });

      const contentType = response.headers.get('content-type');
      let responseData;

      if (contentType && contentType.includes('application/json')) {
        responseData = await response.json();
      } else {
        responseData = { message: await response.text() };
      }

      if (response.ok) {
        setResponse({
          status: response.status,
          statusText: response.statusText,
          data: responseData
        });
      } else {
        setError({
          status: response.status,
          statusText: response.statusText,
          data: responseData
        });
      }
    } catch (err) {
      setError({
        message: `Network error: ${err.message}`,
        type: 'network'
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleGetTest = async () => {
    setIsLoading(true);
    setResponse(null);
    setError(null);
    const email = formData.email;

    try {
      const response = await fetch('http://localhost:3000/crud/get_by_email', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json', // Fixed: Added Content-Type header
          'Accept': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify({email})
      });

      const responseData = await response.json();

      if (response.ok) {
        setResponse({
          status: response.status,
          statusText: response.statusText,
          data: responseData
        });
      } else {
        setError({
          status: response.status,
          statusText: response.statusText,
          data: responseData
        });
      }
    } catch (err) {
      setError({
        message: `Network error: ${err.message}`,
        type: 'network'
      });
    } finally {
      setIsLoading(false);
    }
  };

  const clearResults = () => {
    setResponse(null);
    setError(null);
  };

  return (
    <div className="max-w-2xl mx-auto p-6 bg-white rounded-lg shadow-lg">
      <h1 className="text-2xl font-bold mb-6 text-gray-800">Axum API Test Client</h1>
      
      <div className="space-y-4 mb-6">
        <div>
          <label htmlFor="email" className="block text-sm font-medium text-gray-700 mb-1">
            Email {/* Fixed: Changed from Username to Email */}
          </label>
          <input
            type="email" 
            id="email" 
            name="email" 
            value={formData.email}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Enter email" 
            required
          />
        </div>

        <div>
          <label htmlFor="password" className="block text-sm font-medium text-gray-700 mb-1">
            Password
          </label>
          <input
            type="password"
            id="password"
            name="password"
            value={formData.password}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Enter password"
            required
          />
        </div>

        <div className="flex gap-3">
          <button
            type="button"
            onClick={handlePostRequest}
            disabled={isLoading}
            className="flex items-center justify-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isLoading ? (
              <Loader2 className="w-4 h-4 animate-spin mr-2" />
            ) : (
              <Send className="w-4 h-4 mr-2" />
            )}
            Save Credentials
          </button>

          <button
            type="button"
            onClick={handleGetTest}
            disabled={isLoading}
            className="flex items-center justify-center px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isLoading ? (
              <Loader2 className="w-4 h-4 animate-spin mr-2" />
            ) : (
              <CheckCircle className="w-4 h-4 mr-2" />
            )}
            Get by Email
          </button>

          {(response || error) && (
            <button
              type="button"
              onClick={clearResults}
              className="px-4 py-2 bg-gray-500 text-white rounded-md hover:bg-gray-600 transition-colors"
            >
              Clear
            </button>
          )}
        </div>
      </div>

      {/* Success Response */}
      {response && (
        <div className="mb-4 p-4 bg-green-50 border border-green-200 rounded-md">
          <div className="flex items-center mb-2">
            <CheckCircle className="w-5 h-5 text-green-600 mr-2" />
            <h3 className="text-lg font-semibold text-green-800">Success Response</h3>
          </div>
          <div className="text-sm text-green-700">
            <p><strong>Status:</strong> {response.status} {response.statusText}</p>
          </div>
          <div className="mt-3">
            <h4 className="font-medium text-green-800 mb-1">Response Data:</h4>
            <pre className="bg-green-100 p-3 rounded text-xs overflow-x-auto text-green-900">
              {JSON.stringify(response.data, null, 2)}
            </pre>
          </div>
        </div>
      )}

      {/* Error Response */}
      {error && (
        <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
          <div className="flex items-center mb-2">
            <XCircle className="w-5 h-5 text-red-600 mr-2" />
            <h3 className="text-lg font-semibold text-red-800">Error Response</h3>
          </div>
          {error.status && (
            <div className="text-sm text-red-700">
              <p><strong>Status:</strong> {error.status} {error.statusText}</p>
            </div>
          )}
          <div className="mt-3">
            <h4 className="font-medium text-red-800 mb-1">Error Details:</h4>
            <pre className="bg-red-100 p-3 rounded text-xs overflow-x-auto text-red-900">
              {JSON.stringify(error.data || error.message, null, 2)}
            </pre>
          </div>
        </div>
      )}

      {/* Request Info */}
      <div className="mt-6 p-4 bg-gray-50 border border-gray-200 rounded-md">
        <h3 className="text-lg font-semibold text-gray-800 mb-2">Request Information</h3>
        <div className="text-sm text-gray-600 space-y-1">
          <p><strong>Save URL:</strong> http://localhost:3000/crud/save_credentials</p>
          <p><strong>Get URL:</strong> http://localhost:3000/crud/get_by_email</p>
          <p><strong>Headers:</strong> Content-Type: application/json, Accept: application/json</p>
          <p><strong>Credentials:</strong> include (for cookies/auth)</p>
        </div>
        <div className="mt-3">
          <h4 className="font-medium text-gray-700 mb-1">Current Request Body:</h4>
          <pre className="bg-gray-100 p-3 rounded text-xs overflow-x-auto text-gray-800">
            {JSON.stringify(formData, null, 2)}
          </pre>
        </div>
      </div>
    </div>
  );
}

export default App;