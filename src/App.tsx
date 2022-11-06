import React, { useEffect, useState } from 'react';
import init, { add } from "wasm-lib";
import command_list from "wasm-lib";
import './App.css';
// import { BrowserRouter as Router, Route, Link } from "react-router-dom";
import Route from './components/Route';
import LinuxGame from './components/LinuxGame';
import VimGame from './components/VimGame';
import InterviewQs from './components/InterviewQs';

function MainMenu() {
	return (

		<div className="Game-choice-container">
				<button className="Game-choice-button">Linux</button>
				<button className="Game-choice-button">Vim</button>
				<button className="Game-choice-button">Interview</button>
		
		</div>
	);
}


export default function App(props: any) {   
  	const [ans, setAns] = useState(0); 
  	const [view, setView] = useState<any>("");

  	useEffect(() => {
    		init().then(() => {
      				setAns(add(5, 5));
    			})  
  		}, []) 
 	

	return (
    		<div className="App">
      			<header className="App-header">
				<h3>MEMORIZO</h3>
      			</header>
			
      			<p>rust_func: {ans}</p>

      			<div className="Game-window">
      			</div>
    		</div>
	);  
}


// export default function App(props: any) {   
//   	const [ans, setAns] = useState(0); 
//   	const [view, setView] = useState<any>("");
// 
//   	useEffect(() => {
//     		init().then(() => {
//       				setAns(add(5, 5));
//     			})  
//   		}, []) 
//   
// 
// 	return (
//     		<div className="App">
//       			<header className="App-header">
// 				<h3>MEMORIZO</h3>
//       			</header>
//       			<div className="Game-window">
//       				<p>rust_func: {ans}</p>
// 				<h3>this is the game</h3>
// 			<div className="Game-choice-container">
// 				<button className="Game-choice-button" onClick={setView}>Linux</button>
// 				<button className="Game-choice-button" onClick={setView}>Vim</button>
// 				<button className="Game-choice-button" onClick={setView}>Interview</button>
// 		
// 	</div>
//       </div>
//     </div>
//   );  
// }
