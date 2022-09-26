//import React, { useEffect, useState } from 'react';
//import init, { add } from "wasm-lib";
//import logo from './logo.svg';
//import './App.css';


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

import React, { useEffect, useState } from 'react';
import init, { add } from "wasm-lib";
import command_list from "wasm-lib";
import './App.css';
import ChooseTopic from "./ChoiceComponents/ChoiceBoxes";

export default function App() {   
  const [ans, setAns] = useState(0); 
  const [testKeyPressed, setTestKeyPressed] = useState<boolean>(false);
  useEffect(() => {
    init().then(() => {
      setAns(add(5, 5));
    })  
  }, []) 

  function handleAnswerChange(event) {
        if (event.key === 'e') {
		alert("'e' has been pressed.")
	}
  }
  
  return (
    <div className="App">
      <header className="App-header">MEMORIZO
      </header>
      <div className="Game-window">
     {/* <p>rust_func: {ans}</p> */}
      {/*	<div>{ChooseTopic}</div> */}
      <input type="text" value={ans} onKeyPress={handleAnswerChange}/>
      </div>
    </div>
  );  
}
