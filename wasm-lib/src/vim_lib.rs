use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
pub fn add_test() {
    assert_eq!(5 + 5, add(5, 5)),
}

/////////
//#[wasm-bindgen]
//pub mod cat {
//    pub fn meow() {
//        crate::dog::woof(),
//    }
//}
//
//#[wasm-bindgen]
//pub mod dog {
//    pub fn woof() {
//        println!("Woof Woof!"),
//    }
//}
//
//#[wasm-bindgen]
//pub fn test_one() {
//    cat::meow(),
//}
////////////

#[wasm_bindgen]
pub mod command_list() {
    pub struct save_and_exit {
         // :e myfile - Opens "myfile" for editing
         let 0: (String, Vec<&str>) = ("Opens 'myfile' for editing", vec![":", "e"]),
         // :w - Save file
         let 1: (String, Vec<&str>) = ("Save file", vec![":", "w"]),
         // :sav myfile.txt
         let 2: (String, Vec<&str>) = ("Save myfile.txt", vec![":", "s", "a", "v"]),
         // 😡 - Write changes to file and exit
         // ^ Not sure how to implement emojis rn, will deal with that later
         // :q! - Quit without saving changes
         let 4: (String, Vec<&str>) = ("Quit without saving changes", vec![":", "q", "!"]),
         // :q - quit vim
         let 5: (String, Vec<&str>) = ("Quit vim", vec![":", "q"]),

    }

    pub struct find_and_replace() {
        // y    Copy the selected text
        let 0: (String, Vec<&str>) = ("Copy the selected text", vec!["y"]),
        // p    Paste the selected text
        let 1: (String, Vec<&str>) = ("Paste the selected text", vec!["d", "d"]),
        // dd   Cut the current line
        let 2: (String, Vec<&str>) = ("Cut the current line", vec!["d", "d"]),
        // yy   Copy the current line
        let 3: (String, Vec<&str>) = ("Copy the current line", vec!["y", "y"]),
        // y$   Copy to EOL
        let 4: (String, Vec<&str>) = ("Copy to EOL", vec!["y", "$"]),
        // D    Cut to EOL
        let 5: (String, Vec<&str>) = ("Cut to EOL", vec!["D"]),
    }
    
    pub struct change_case_commands() {
        // Vu   Lowercase the entire line
        let 0: (String, Vec<&str>) = ("Lowercase the entire line", vec!["V", "u"]),
        //VU    Uppercase the entire line
        let 1: (String, Vec<&str>) = ("Uppercase the entire line", vec!["V", "U"]),
        //g~~   Invert selected case
        let 2: (String, Vec<&str>) = ("Invert the selected case", vec!["g", "~", "~"]),
        //vEU   Switch selected word to uppercase
        let 3: (String, Vec<&str>) = ("Switch selected word to uppercase", vec![]),
        //vE~   Modify the current word case
        let 4: (String, Vec<&str>) = ("Modify the current word case", vec!["v", "E", "~"]),
        // ggguG Set all text to lowercase
        let 5: (String, Vec<&str>) = ("Set all text to lowercase", vec!["g", "g", "g", "u", "G"]),
        //gggUG Set all text to uppercase
        let 6: (String, Vec<&str>) = ("Set all text to uppercase", vec![]),
        //:set ignorecase       Ignore case when searching text
        let 7: (String, Vec<&str>) = ("Ignore case when searching text", vec![]),
        //:set smartcase        Ignore case in search, but not if an uppercase letter is used
        let 8: (String, Vec<&str>) = ("Ignore case in search, but not if an uppercase letter is used", vec![]),
        //:%s/\<./\u&/g Sets the first letter of each word to uppercase
        let 9: (String, Vec<&str>) = ("Sets the first letter of each word to uppercase", vec![]),
        //:%s/\<./\l&/g Sets the first letter of each word to lowercase
        let 10: (String, Vec<&str>) = ("Sets the first letter of each word to lowercase", vec![]),
        //:%s/.*/\u&    Sets the first letter of each line to uppercase
        let 11: (String, Vec<&str>) = ("Sets the first letter of each line to uppercase", vec![]),
        //:%s/.*/\l&    Sets the first letter of each line to lowercase
        let 12: (String, Vec<&str>) = ("Sets the first letter of each line to lowercase", vec![":", "%", "s", "/", ".", "*", "/", "'\'", "l", "&"]),
    }

    pub struct navigate_within_file() {
        // k or Up Arrow        Move the cursor position up one line
        let 0: (String, Vec<&str>) = ("Move the cursor position up one line", vec!["k", "OR", "[up arrow]"]),
        //j or Down Arrow Move the cursor down one line
        let 1: (String, Vec<&str>) = ("Move the cursor down one line", vec!["j", "[down arrow]"),
        //e     Move the cursor to the end of the word
        let 2: (String, Vec<&str>) = ("Move the cursor to the end of the first word", vec!["e"]),
        //b     Move the cursor to the beginning of the word
        let 3: (String, Vec<&str>) = ("Move the cursor to the beginning of the word", vec!["b"]),
        //0     Move the cursor to the beginning of the line
        let 4: (String, Vec<&str>) = ("Move the cursor to the beginning of the line", vec!["0"]),
        //G     Move the cursor to EOF
        let 5: (String, Vec<&str>) = ("Move the cursor to the EOF", vec!["G"]),
        //gg    Move the cursor to the beginning of the file
        let 6: (String, Vec<&str>) = ("Move the cursor to the beginning of the file", vec!["g", "g"]),
        //L     Move the cursor to the bottom of the screen
        let 7: (String, Vec<&str>) = ("Move the cursor to the bottom of screen", vec!["L"]),
        //:80   Move the cursor to line number 80
        let 8: (String, Vec<&str>) = ("Move the cursor to line number 80", vec![":", "8", "0"]),
        //%     Move the cursor to matching parenthesis
        let 9: (String, Vec<&str>) = ("Move the cursor to matching parenthesis", vec!["&"]),
        //[[    Move the cursor to function start
        let 10: (String, Vec<&str>) = ("Move the cursor to function start", vec!["[", "["]),
        //[{    Move the cursor to block start
        let 11: (String, Vec<&str>) = ("Move the cursor to block start", vec!["[", "{"]),
    }

    pub struct file_explorer_read_write() {
        // :1,10 w myfile       Saves lines 1 to 10 in myfile
        let 0: (String, Vec<&str>) = ("Save lines 1 to 10 in myfile", vec![":", "1", ",", "10", " ", "w", " ", "m", "y", "f", "i", "l", "e"]),         
        //:1,10 w >> myfile     Appends lines 1 to 10 to myfile
        let 1: (String, Vec<&str>) = ("Appends lines 1 to 10 to myfile", vec![":", "1", ",", "1", "0"]),   
        //:r myfile     Inserts the content of myfile to current file
        let 2: (String, Vec<&str>) = ("Inserts the content of myfile to current file", vec![":", "r"]),
        //:23r myfile   Inserts the content of myfile under line 23
        let 3: (String, Vec<&str>) = ("Inserts the content of myfile under line 23", vec![":", "2", "3", "r"]),
        //:e .  Open the File Explorer
        let 4: (String, Vec<&str>) = ("Open the File Explorer", vec![":", "e", " ", "."]),
        //:Sex  Split window and open File Explorer
        let 5: (String, Vec<&str>) = ("Split window and open File Explorer", vec![":", "S", "e", "x"]),
        //:Sex! Same as :Sex but splits window vertically
        let 6: (String, Vec<&str>) = ("Same as :Sex but splits windows vertically", vec![":", "S", "e", "x", "!"]),
        //:browse e     Graphical File Explorer
        let 7: (String, Vec<&str>) = ("Graphical File Explorer", vec![":", "b", "r", "o", "w", "s", "e", " ", "e"]),
        //:ls   List buffers
        let 8: (String, Vec<&str>) = ("List buffers", vec![":", "l", "s"]),
        //:cd ..        Move to parent or root directory
        let 9: (String, Vec<&str>) = ("Move to parent or root directory", vec![":", "c", "d", " ", ".", "."]),
        //:args List files
        let 10: (String, Vec<&str>) = ("List files", vec![":", "a", "r", "g", "s"]),
        //:args *.php   Open file list with .php extension
        let 11: (String, Vec<&str>) = ("Open file list with .php extension", vec![":", "a", "r", "g", "s", " ", "*", ".", "p", "h", "p"]),
        //:grep something *.php Returns list of .php files containing something
        let 12: (String, Vec<&str>) = ("Returns list of .php files containing something", vec![":", "g", "r", "e", "p"]),
        //gf    Open file name under cursor
        let 13: (String, Vec<&str>) = ("Open file name under cursor", vec!["g", "f"]),
    }

    pub struct interface_commands_tabs_windows() {
        // :tabnew      Opens a new tab
        let 0: (String, Vec<&str>) = ("Opens a new tab", vec![":", "t", "a", "b", "n", "e", "w"]),
        //gt    Go to next tab
        let 1: (String, Vec<&str>) = ("Go to next tab", vec!["g", "t"]),
        //:tabfirst     Go to first tab
        let 2: (String, Vec<&str>) = ("Go to first tab", vec![":", "t", "a", "b", "f", "i", "r", "s", "t"]),
        //:tablast      Go to last tab
        let 3: (String, Vec<&str>) = ("Go to last tab", vec![":", "t", "a", "b", "l", "a", "s", "t"]),
        //:tabm n(position)     Rearrange open tabs
        let 4: (String, Vec<&str>) = ("Re-arrange open tabs", vec![":", "t", "a", "b", "m", " ", "n", "[position]"]),
        //:tabdo %s/foo/bar/g   Execute same command in all tabs
        let 5: (String, Vec<&str>) = ("Execute same command in all tabs", vec!["t", "a", "b", "d", "o", " ", "%", "s", "/", "f", "o", "o", "/", "b", "a", "r", "/", "g"]),
        //:tab ball     Puts all open files in different tabs
        let 6: (String, Vec<&str>) = ("Puts all open files in different tabs", vec![":", "t", "a", "b", " ", "b", "a", "l", "l"]),
        //:new myfile.txt       Edit myfile.txt in new window
        let 7: (String, Vec<&str>) = ("Edit myfile.txt in new window", vec![":", "n", "e", "w", " ", "m", "y", "f", "l", "e", ".", "t", "x", "t"]),
        //:e filename   Edit filename in current window
        let 8: (String, Vec<&str>) = ("Edit filename in current window", vec![":", "e", " ", "f", "i", "l", "e", "n", "a", "m", "e"]),
        //:split myfile Split the window and open myfile
        let 9: (String, Vec<&str>) = ("Split the window and open myfile", vec![":", "s", "p", "l", "i", "t", " ", "m", "y", "f", "i", "l", "e"]),
        //ctrl-w + Up arrow   Puts cursor in top window
        let 10: (String, Vec<&str>) = ("Puts cursor in top window", vec!["c", "t", "r", "l", " ", "+", "[up arrow]"),
        //ctrl-w ctrl-w (twice) Puts cursor in next window
        let 11: (String, Vec<&str>) = ("Puts cursor in next window", vec!["[ctrl]", "-", "w", " ", "[ctrl]", "-", "w", "(twice)"]), 
        //ctrl-w_       Maximize current window vertically
        let 12: (String, Vec<&str>) = ("Maximize current window vertically", vec!["[ctrl]", "-", "w"]),
        //ctrl-w|       Maximize current window horizontally
        let 13: (String, Vec<&str>) = ("Maximize current window horizontally", vec!["[ctrl]", "-", "w"]),
        //ctrl-w=       Make all windows of the same size
        let 14: (String, Vec<&str>) = ("Make all windows of the same size", vec![]), 
        //100 ctrl-w+   Add 100 lines to file in current window
        let 15: (String, Vec<&str>) = ("Add 100 lines to file in current window", vec!["1", "0", "0", "[ctrl]", "-", "w", "+"),
        //:vsplit file  Split windows vertically
        let 16: (String, Vec<&str>) = ("Split windows vertically", vec![":", "v", "s", "p", "l", "i", "t"]),
        //:sview file   Split windows vertically (read-only)
        let 17: (String, Vec<&str>) = ("Split windows vertically (read-only)", vec!["s", "v", "i", "e", "w", "f", "i", "l", "e"]),
        //:hide Close current window
        let 18: (String, Vec<&str>) = ("Close current window", vec![":", "h", "i", "d", "e"]),
        //:­nly Close all windows, except the current
        let 19: (String, Vec<&str>) = ("Close all windows (except current)", vec![":", "n", "l", "y"]),
        //:b 4  Open tab #4 in current window
        let 20: (String, Vec<&str>) = ("Open tab #4 in current window", vec![":", "n", "l", "y"]),

    }

    pub struct text_alignment_and_indentation() {
        // :set autoindent      Turn on auto-indentation
        let 0: (String, Vec<&str>) = ("Turn on auto-indentation", vec![]),
        // :%!fmt        Align all the lines
        let 1: (String, Vec<&str>) = ("Align all the lines", vec![]),
        // !}fmt Align all lines at the current position
        let 2: (String, Vec<&str>) = ("Align all lines at the current position", vec![]),
        // 2!!fmt        Align the next two lines
        let 3: (String, Vec<&str>) = ("Align the next two lines", vec![]),
        // :set smartindent      Turn on smart auto-indentation
        let 4: (String, Vec<&str>) = ("Turn on smart auto-indentation", vec![]), 
        // :set shiftwidth=8     Defines 8 spaces as indent size
        let 5: (String, Vec<&str>) = ("Defines 8 spaces as indent size", vec![]),
        // ctrl-t, ctrl-d        Indent and un-indent in Insert Mode
        let 6: (String, Vec<&str>) = ("Indent and un-indent in Insert Mode", vec![]), 
        // >>    Indent the current line
        let 7: (String, Vec<&str>) = ("Indent the current line", vec![]), 
        // <<    Un-indent the current line
        let 8: (String, Vec<&str>) = ("Un-indent the current line", vec![]),
        // =%    Indent the code between parenthesis
        let 9: (String, Vec<&str>) = ("Indent the code between parethesis", vec![]),
        // 1GVG= Indent the entire file
        let 10: (String, Vec<&str>) = ("Indent the current file", vec![]),

    }

    pub struct autocomplete_vim_commands() {
        // Ctrl+n Ctrl+p        Complete the suggested word (Insert Mode)
        let 0: (String, Vec<&str>) = ("Complete the suggested word (Insert Mode)", vec![]),
        // Ctrl+x Ctrl+l        Complete the suggested line
        let 1: (String, Vec<&str>) = ("Complete the suggested line", vec![]),
        // :set dictionary=en   Define en as active dictionary
        let 2: (String, Vec<&str>) = ("Define en as active dictionary", vec![]),
        // Ctrl+x Ctrl+k        Complete with the active dictionary
        let 3: (String, Vec<&str>) = ("Complete with the active dictionary", vec![]),

    }

    pub struct unix_only_vim_commands() {
         // :!get        Execute the get Unix command, then return to Vim
         let 0: (String, Vec<&str>) = ("Execute the get Unix command, then retun to Vim", vec![]),
         // !!get Execute the get Unix command and insert output in current file
         let 1: (String, Vec<&str>) = ("Execute the Unix command and insert output in current file", vec![]),
         //:sh   Return to Unix shell
         let 2: (String, Vec<&str>) = ("Return to Unix shell", vec![]), 
         //$exit Exit the Unix shell and return to Vim
         let 3: (String, Vec<&str>) = ("Exit the Unix shell and return to Vim", vec![]),
         // m {q-p}      Marks the current position as {q-p}
         let 4: (String, Vec<&str>) = ("Mark the current position as {q-p}", vec![]),
         //‘ {q-p}       Move to position {q-p} (used after marking)
         let 5: (String, Vec<&str>) = ("Move to position {q-p} (used after marking)", vec![]),
         //“     Move to previously marked position
         let 6: (String, Vec<&str>) = ("Move to previously marked position", vec![]), 
         //:ab mail hi@designbombs.com   Define mail as abbreviation of hi@designbombs.com
         let 7: (String, Vec<&str>) = ("Define mail as abbreviation of hi@designbombs.com", vec![]),
    }
}
