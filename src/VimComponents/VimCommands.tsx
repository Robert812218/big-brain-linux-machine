// SAVE AND EXIT COMMANDS:
//
// :e myfile - Opens "myfile" for editing
// :w - Save file
// :sav myfile.txt
// 😡 - Write changes to file and exit
// :q! - Quit without saving changes
// :q - quit vim
//
//  FIND AND REPLACE COMMANDS:
//
//  /xyz - search xyz from top to bottom
//  ?xyz - search xyz from bottom to top
//  * - Search the text that is under the cursor
//  /\ctext - Search for text (case-insensitive)
//  /ra[av]i - Search for raai or ravi
//  /abc\|xyz - Search for abc or xyz
//  /\<\d\d\d\d\>	Search for exactly 4 digits
// /^\n\{3}	Search for 3 empty lines
// :bufdo /searchstr/	Search in all open files globally
// bufdo %s/findme/replaceme/g	Search findme in all the open buffers and replace it with replaceme
// :%s/x/y/g	Replace all occurrences of x by y in file
// :%s/qwerty/ytrewq/gi	Replace qwerty by ytrewq, case-insensitive
// :%s/x/y/gc	Replace all occurrences after confirmation
// :%s/^/Begin/g	Replace the beginning of each line by Begin
// :%s/$/End/g	Replace the ending of each line by End
// :%s/x/y/gi	Replace x by y, case-insensitive
// :%s/ *$//g	Search and delete all white spaces
// :g/myname/d	Search and delete all lines containing myname
// :v/myname/d	Delete all lines not containing myname
// :s/John/Doe/	Replace the first occurrence of John by Doe in current line
// :s/John/Doe/g	Replace John by Doe in current line
// :%s/John/Doe/g	Replace John by Doe in all files
// :%s/^M//g	Search and delete DOS carriage returns (^M)
// :%s/\r/\r/g	Replace DOS carriage returns with regular return key
// :%s#<[^>]\+>##g	Search and delete HTML tags but keep text
// :%s/^\(.*\)\n\1$/\1/	Search and delete duplicate lines
// Ctrl+a	Increments number under the cursor
// Ctrl+x	Decrements number under cursor
// 
// CUT, COPY AND PASTE COMMANDS:
//
// y	Copy the selected text
// p	Paste the selected text
// dd	Cut the current line
// yy	Copy the current line
// y$	Copy to EOL
// D	Cut to EOL
//
// CHANGE CASE COMMANDS:
//
// Vu	Lowercase the entire line
//VU	Uppercase the entire line
//g~~	Invert selected case
//vEU	Switch selected word to uppercase
//vE~	Modify the current word case
//ggguG	Set all text to lowercase
//gggUG	Set all text to uppercase
//:set ignorecase	Ignore case when searching text
//:set smartcase	Ignore case in search, but not if an uppercase letter is used
//:%s/\<./\u&/g	Sets the first letter of each word to uppercase
//:%s/\<./\l&/g	Sets the first letter of each word to lowercase
//:%s/.*/\u&	Sets the first letter of each line to uppercase
//:%s/.*/\l&	Sets the first letter of each line to lowercase
//
// NAVIGATION WITHIN THE FILE:
//
// k or Up Arrow	Move the cursor position up one line
//j or Down Arrow	Move the cursor down one line
//e	Move the cursor to the end of the word
//b	Move the cursor to the beginning of the word
//0	Move the cursor to the beginning of the line
//G	Move the cursor to EOF
//gg	Move the cursor to the beginning of the file
//L	Move the cursor to the bottom of the screen
//:80	Move the cursor to line number 80
//%	Move the cursor to matching parenthesis
//[[	Move the cursor to function start
//[{	Move the cursor to block start
//
// FILE READ/WRITE AND FILE EXPLORER
//
// :1,10 w myfile	Saves lines 1 to 10 in myfile
//:1,10 w >> myfile	Appends lines 1 to 10 to myfile
//:r myfile	Inserts the content of myfile to current file
//:23r myfile	Inserts the content of myfile under line 23
//:e .	Open the File Explorer
//:Sex	Split window and open File Explorer
//:Sex!	Same as :Sex but splits window vertically
//:browse e	Graphical File Explorer
//:ls	List buffers
//:cd ..	Move to parent or root directory
//:args	List files
//:args *.php	Open file list with .php extension
//:grep something *.php	Returns list of .php files containing something
//gf	Open file name under cursor
//
// INTERFACE-RELATED VIM COMMANDS (TABS AND WINDOWS)
//
// :tabnew	Opens a new tab
//gt	Go to next tab
//:tabfirst	Go to first tab
//:tablast	Go to last tab
//:tabm n(position)	Rearrange open tabs
//:tabdo %s/foo/bar/g	Execute same command in all tabs
//:tab ball	Puts all open files in different tabs
//:new myfile.txt	Edit myfile.txt in new window
//:e filename	Edit filename in current window
//:split myfile	Split the window and open myfile
//ctrl-w + Up arrow	Puts cursor in top window
//ctrl-w ctrl-w (twice)	Puts cursor in next window
//ctrl-w_	Maximize current window vertically
//ctrl-w|	Maximize current window horizontally
//ctrl-w=	Make all windows of the same size
//100 ctrl-w+	Add 100 lines to file in current window
//:vsplit file	Split windows vertically
//:sview file	Split windows vertically (read-only)
//:hide	Close current window
//:­nly	Close all windows, except the current
//:b 4	Open tab #4 in current window
//
// TEXT ALIGNMENT AND INDENTATION
//
// :set autoindent	Turn on auto-indentation
//:%!fmt	Align all the lines
//!}fmt	Align all lines at the current position
//2!!fmt	Align the next two lines
//:set smartindent	Turn on smart auto-indentation
//:set shiftwidth=8	Defines 8 spaces as indent size
//ctrl-t, ctrl-d	Indent and un-indent in Insert Mode
//>>	Indent the current line
//<<	Un-indent the current line
//=%	Indent the code between parenthesis
//1GVG=	Indent the entire file
//
//
// AUTOCOMPLETE VIM COMMANDS 
//
// Ctrl+n Ctrl+p	Complete the suggested word (Insert Mode)
// Ctrl+x Ctrl+l	Complete the suggested line
// :set dictionary=en	Define en as active dictionary
// Ctrl+x Ctrl+k	Complete with the active dictionary
//
// UNIX-ONLY VIM COMMANDS: 
//
// :!get	Execute the get Unix command, then return to Vim
//!!get	Execute the get Unix command and insert output in current file
//:sh	Return to Unix shell
//$exit	Exit the Unix shell and return to Vim
//
// m {q-p}	Marks the current position as {q-p}
//‘ {q-p}	Move to position {q-p} (used after marking)
//“	Move to previously marked position
//:ab mail hi@designbombs.com	Define mail as abbreviation of hi@designbombs.com
//
//

export default function VimCommands() {
}
