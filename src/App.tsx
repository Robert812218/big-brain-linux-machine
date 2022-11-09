import React, { useEffect, useState } from 'react';
import init, { add } from "wasm-lib";
import command_list from "wasm-lib";
import './App.css';
// import { BrowserRouter as Router, Route, Link } from "react-router-dom";
import Route from './components/Route';
import LinuxGame from './components/LinuxGame';
import VimGame from './components/VimGame';
import InterviewQs from './components/InterviewQs';
import Header from './components/Header';


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
    		<div className="App-container">

      			<div className="App-header">
				<header>MEMORIZO</header>
				<nav>
					<Header />
					<div className="Route-component">
						<Route path="/">
						
						</Route>
					</div>
					
					<div className="Route-component">	
						<Route path="/LinuxGame">
							<LinuxGame />
						</Route>
					</div>

					<div className="Route-component">
						<Route path="/VimGame">
							<VimGame />
						</Route>
					</div>

					<div className="Route-component">
						<Route path="/InterviewQs">
							<InterviewQs />
						</Route>
					</div>
				</nav>
      			</div>
			
			<div className="Game-container">
				
			</div>

			<div>
			
			</div>

      			<p>rust_func: {ans}</p>

    		</div>
	);  
}


