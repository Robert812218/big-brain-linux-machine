import React from "react";
import Link from "./Link";

const Header = () => {
	return (
		<div className="ui secondary pointing menu">
			<Link href="/">Menu</Link>

			<Link href="/LinuxGame">Linux</Link>

			<Link href="/VimGame">Vim</Link>

			<Link href="/InterviewQs">InterviewQs</Link>
		</div>
	);
}

export default Header;
