import React, { useEffect, useState } from 'react';
import init, { add } from "wasm-lib";
import command_list from "wasm-lib";
import './App.css';

export default function App(props: any) {   
  const [ans, setAns] = useState(0); 
  useEffect(() => {
    init().then(() => {
      setAns(add(5, 5));
    })  
  }, []) 
  
  function RenderVim() {
    return <h4>VIM GAME</h4>
  }

  function RenderLinux() {
    return <h4>LINUX GAME</h4>
  }

  return (
    <div className="App">
      <header className="App-header">
	<h3>MEMORIZO</h3>
      </header>
      <div className="Game-window">
      {/* <p>rust_func: {ans}</p> */}
      {/*	<div>{ChooseTopic}</div> */}
	<h3>this is the game</h3>
        <h2>Linux or Vim?</h2>
	<div className="Game-choice-container">
	  <button className="Game-choice-button" onClick={() => 
	         alert("You chose Linux") 
			        
	  }>LINUX</button> 
	  <button className="Game-choice-button" onClick={() => 
	        alert("You chose Vim") 
	  }> VIM </button>
	</div>
      </div>
    </div>
  );  
}

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

