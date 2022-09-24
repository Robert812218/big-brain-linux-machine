import React, { useEffect, useState } from 'react';
import init, { add } from "wasm-lib";
import logo from './logo.svg';
import './App.css';


//export default function App() {  
//  const [ans, setAns] = useState(0);
//  useEffect(() => {
//    init().then(() => {
//      setAns(add(1, 1));
//    })
//  }, [])
//  return (
//    <div className="App">
//      <header className="App-header">
//      </header>
//      <div className="game-window">
//	<div className="text-question"></div>
//        <p>1 + 1 = {ans}</p>
//	<div className="text-entry-box"></div>
//      </div>
//    </div>
//  );
//}

export default function App() {   
  const [ans, setAns] = useState(0);
  
  useEffect(() => {
    init().then(() => {
      setAns(add(1, 1));
    })  
  }, []) 
  return (
    <div className="App">
      <header className="App-header">
      </header>
      <div className="game-window">
        <div className="text-question"></div>
        <p>1 + 1 = {ans}</p>
        <div className="text-entry-box"></div>
      </div>
    </div>
  );  
}
