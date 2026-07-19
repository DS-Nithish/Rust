import React, { useState } from "react";
import "./App.css";
import { Greeting, Welcome, Bye } from "./Greeting";

function App() {
  const [name, setName] = useState("");

  return (
    <div className="container">
      <input
        type="text"
        placeholder="Enter your name"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />

      <Greeting name={name} />
      <Welcome />
      <Bye name={name} />
    </div>
  );
}

export default App;