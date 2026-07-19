import React, { useState } from "react";
import "./App.css"; // Import CSS

function CounterApp() {
  const [count, setCount] = useState(0);
  const [name, setName] = useState("");
  const [showCounter, setShowCounter] = useState(true);

  return (
    <div className="container">
      {!showCounter ? (
        <div>
          <h1>Enter Your Name</h1>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Enter your name"
          />
          <br />
          <button onClick={() => setShowCounter(true)} disabled={!name}>
            Go to Counter
          </button>
        </div>
      ) : (
        <div>
          <h2>Hello, {name}! You can use the counter</h2>

          <div className="counter">{count}</div>

          <button onClick={() => setCount(count + 1)}>Increment</button>
          <button onClick={() => setCount(count - 1)}>Decrement</button>

          <br />
          <button onClick={() => setShowCounter(false)}>Back</button>
        </div>
      )}
    </div>
  );
}

export default CounterApp;
