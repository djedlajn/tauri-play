import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [url, setUrl] = useState("https://google.com");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [history, setHistory] = useState<string[]>(["https://google.com"]);

  useEffect(() => {
    // Listen for URL change events from the right window
    const unlisten = listen('url-changed', (event) => {
      const newUrl = event.payload as string;
      addToHistory(newUrl);
    });

    return () => {
      unlisten.then(unlistenFn => unlistenFn());
    };
  }, []);

  const addToHistory = (newUrl: string) => {
    setHistory(prevHistory => {
      // Don't add duplicate consecutive entries
      if (prevHistory.length > 0 && prevHistory[0] === newUrl) {
        return prevHistory;
      }
      // Keep only the last 10 entries
      return [newUrl, ...prevHistory].slice(0, 10);
    });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Reset error state
    setError(null);
    
    // Basic URL validation
    let urlToNavigate = url;
    if (!url.startsWith("http://") && !url.startsWith("https://")) {
      urlToNavigate = "https://" + url;
      setUrl(urlToNavigate);
    }
    
    try {
      setIsLoading(true);
      // Navigate the right window to the new URL
      await invoke("navigate_webviews", { url: urlToNavigate });
      // Add to history
      addToHistory(urlToNavigate);
    } catch (error) {
      console.error("Error navigating webviews:", error);
      setError(typeof error === 'string' ? error : "Failed to navigate to the URL");
    } finally {
      setIsLoading(false);
    }
  };

  const handleHistoryClick = async (historyUrl: string) => {
    setUrl(historyUrl);
    try {
      setIsLoading(true);
      await invoke("navigate_webviews", { url: historyUrl });
    } catch (error) {
      console.error("Error navigating to history URL:", error);
      setError(typeof error === 'string' ? error : "Failed to navigate to the URL");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="container">
      <h1>Web View Navigator</h1>
      <div className="row">
        <form onSubmit={handleSubmit}>
          <input
            id="url-input"
            type="text"
            placeholder="Enter URL (e.g., google.com)"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            disabled={isLoading}
          />
          <button type="submit" disabled={isLoading}>
            {isLoading ? "Loading..." : "Navigate"}
          </button>
        </form>
      </div>
      
      {error && (
        <div className="row">
          <div className="card error">
            <p>{error}</p>
          </div>
        </div>
      )}
      
      <div className="row">
        <div className="card">
          <p>
            Enter a website URL above and click "Navigate" to load it in the right window.
          </p>
          <p>
            The URL will automatically be prefixed with https:// if needed.
          </p>
        </div>
      </div>

      <div className="row">
        <div className="history-container">
          <h2>Browsing History</h2>
          {history.length > 0 ? (
            <ul className="history-list">
              {history.map((historyUrl, index) => (
                <li key={index} className="history-item">
                  <button
                    className="history-button"
                    onClick={() => handleHistoryClick(historyUrl)}
                    title={historyUrl}
                  >
                    {historyUrl}
                  </button>
                </li>
              ))}
            </ul>
          ) : (
            <p>No browsing history yet.</p>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
