import React, { useState } from "react";
import './App.css';


const questionsData = [
  {
    question: "What is the capital of France?",
    options: ["Paris", "London", "Berlin", "Rome"],
    answer: "Paris"
  },
  {
    question: "Who wrote 'Hamlet'?",
    options: ["Shakespeare", "Charles Dickens", "J.K. Rowling", "Mark Twain"],
    answer: "Shakespeare"
  },
  {
    question: "What is the smallest planet in our solar system?",
    options: ["Earth", "Venus", "Mars", "Mercury"],
    answer: "Mercury"
  },
  {
    question: "What is the speed of light?",
    options: ["300,000 km/s", "150,000 km/s", "100,000 km/s", "200,000 km/s"],
    answer: "300,000 km/s"
  },
  {
    question: "What is the chemical symbol for water?",
    options: ["O2", "H2O", "CO2", "H2SO4"],
    answer: "H2O"
  },
  {
    question: "What year did World War II end?",
    options: ["1939", "1945", "1918", "1965"],
    answer: "1945"
  },
  {
    question: "Who painted the Mona Lisa?",
    options: ["Vincent van Gogh", "Pablo Picasso", "Leonardo da Vinci", "Claude Monet"],
    answer: "Leonardo da Vinci"
  },
  {
    question: "What is the largest mammal?",
    options: ["Elephant", "Blue Whale", "Giraffe", "Shark"],
    answer: "Blue Whale"
  },
  {
    question: "Which planet is known as the Red Planet?",
    options: ["Jupiter", "Mars", "Saturn", "Neptune"],
    answer: "Mars"
  },
  {
    question: "What is the hardest natural substance on Earth?",
    options: ["Iron", "Diamond", "Gold", "Platinum"],
    answer: "Diamond"
  }
];


function App() {
  const shuffledQuestions = questionsData.sort(() => 0.5 - Math.random()).slice(0, 5);
  const [currentQuestionIndex, setCurrentQuestionIndex] = useState(0);
  const [score, setScore] = useState(0);
  const [showScore, setShowScore] = useState(false);


  const handleAnswerOptionClick = (selectedOption) => {
    const currentQuestion = shuffledQuestions[currentQuestionIndex];
    if (selectedOption === currentQuestion.answer) {
      setScore(score + 1);
    }
    const nextQuestion = currentQuestionIndex + 1;
    if (nextQuestion < shuffledQuestions.length) {
      setCurrentQuestionIndex(nextQuestion);
    } else {
      setShowScore(true);
    }
  };
  return (
    <div style={{ textAlign: "center", marginTop: "50px" }}>
      <h1>Quiz App</h1>
      {showScore ? (
        <div>
          <h2>Your Score: {score} / {shuffledQuestions.length}</h2>
        </div>
      ) : (
        <>
          <div>
            <h3>Question {currentQuestionIndex + 1}</h3>
            <h4>{shuffledQuestions[currentQuestionIndex].question}</h4>
          </div>
          <div>
            {shuffledQuestions[currentQuestionIndex].options.map((option, index) => (
              <button
                key={index}
                onClick={() => handleAnswerOptionClick(option)}
                style={{
                  margin: "5px",
                  padding: "10px",
                  fontSize: "16px",
                  cursor: "pointer",
                  backgroundColor: "#f0f0f0",
                  color: "blue",
                  border: "1px solid #ccc",
                  borderRadius: "4px"
                }}
              >
                {option}
              </button>
            ))}
          </div>
        </>
      )}
    </div>
  );
}


export default App;
