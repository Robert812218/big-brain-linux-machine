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
         let 4: (String, Vec<&str>) = ("", vec![]),
         // :q - quit vim
         let 5: (String, Vec<&str>) = ("", vec![]),

    }

    pub struct find_and_replace() {
        // y    Copy the selected text
        let 0: (String, Vec<&str>) = ("Copy the selected text", vec![]),
        // p    Paste the selected text
        let 1: (String, Vec<&str>) = ("", vec![]),
        // dd   Cut the current line
        let 2: (String, Vec<&str>) = ("", vec![]),
        // yy   Copy the current line
        let 3: (String, Vec<&str>) = ("", vec![]),
        // y$   Copy to EOL
        let 4: (String, Vec<&str>) = ("", vec![]),
        // D    Cut to EOL
        let 5: (String, Vec<&str>) = ("", vec![]),
    }
    
    pub struct change_case_commands() {
        // Vu   Lowercase the entire line
        let 0: (String, Vec<&str>) = ("", vec![]),
        //VU    Uppercase the entire line
        let 1: (String, Vec<&str>) = ("", vec![]),
        //g~~   Invert selected case
        let 2: (String, Vec<&str>) = ("", vec![]),
        //vEU   Switch selected word to uppercase
        let 3: (String, Vec<&str>) = ("", vec![]),
        //vE~   Modify the current word case
        let 4: (String, Vec<&str>) = ("", vec![]),
        // ggguG Set all text to lowercase
        let 5: (String, Vec<&str>) = ("", vec![]),
        //gggUG Set all text to uppercase
        let 6: (String, Vec<&str>) = ("", vec![]),
        //:set ignorecase       Ignore case when searching text
        let 7: (String, Vec<&str>) = ("", vec![]),
        //:set smartcase        Ignore case in search, but not if an uppercase letter is used
        let 8: (String, Vec<&str>) = ("", vec![]),
        //:%s/\<./\u&/g Sets the first letter of each word to uppercase
        let 9: (String, Vec<&str>) = ("", vec![]),
        //:%s/\<./\l&/g Sets the first letter of each word to lowercase
        let 10: (String, Vec<&str>) = ("", vec![]),
        //:%s/.*/\u&    Sets the first letter of each line to uppercase
        let 11: (String, Vec<&str>) = ("", vec![]),
        //:%s/.*/\l&    Sets the first letter of each line to lowercase
        let 12: (String, Vec<&str>) = ("", vec![]),
    }

    pub struct navigate_within_file() {
        // k or Up Arrow        Move the cursor position up one line
        let 0: (String, Vec<&str>) = ("", vec![]),
        //j or Down Arrow Move the cursor down one line
        let 1: (String, Vec<&str>) = ("", vec![]),
        //e     Move the cursor to the end of the word
        let 2: (String, Vec<&str>) = ("", vec![]),
        //b     Move the cursor to the beginning of the word
        let 3: (String, Vec<&str>) = ("", vec![]),
        //0     Move the cursor to the beginning of the line
        let 4: (String, Vec<&str>) = ("", vec![]),
        //G     Move the cursor to EOF
        let 5: (String, Vec<&str>) = ("", vec![]),
        //gg    Move the cursor to the beginning of the file
        let 6: (String, Vec<&str>) = ("", vec![]),
        //L     Move the cursor to the bottom of the screen
        let 7: (String, Vec<&str>) = ("", vec![]),
        //:80   Move the cursor to line number 80
        let 8: (String, Vec<&str>) = ("", vec![]),
        //%     Move the cursor to matching parenthesis
        let 9: (String, Vec<&str>) = ("", vec![]),
        //[[    Move the cursor to function start
        let 10: (String, Vec<&str>) = ("", vec![]),
        //[{    Move the cursor to block start
        let 11: (String, Vec<&str>) = ("", vec![]),
    }

    pub struct file_explorer_read_write() {
        // :1,10 w myfile       Saves lines 1 to 10 in myfile
        let 0: (String, Vec<&str>) = ("", vec![]),         
        //:1,10 w >> myfile     Appends lines 1 to 10 to myfile
        let 1: (String, Vec<&str>) = ("", vec![]),   
        //:r myfile     Inserts the content of myfile to current file
        let 2: (String, Vec<&str>) = ("", vec![]),
        //:23r myfile   Inserts the content of myfile under line 23
        let 3: (String, Vec<&str>) = ("", vec![]),
        //:e .  Open the File Explorer
        let 4: (String, Vec<&str>) = ("", vec![]),
        //:Sex  Split window and open File Explorer
        let 5: (String, Vec<&str>) = ("", vec![]),
        //:Sex! Same as :Sex but splits window vertically
        let 6: (String, Vec<&str>) = ("", vec![]),
        //:browse e     Graphical File Explorer
        let 7: (String, Vec<&str>) = ("", vec![]),
        //:ls   List buffers
        let 8: (String, Vec<&str>) = ("", vec![]),
        //:cd ..        Move to parent or root directory
        let 9: (String, Vec<&str>) = ("", vec![]),
        //:args List files
        let 10: (String, Vec<&str>) = ("", vec![]),
        //:args *.php   Open file list with .php extension
        let 11: (String, Vec<&str>) = ("", vec![]),
        //:grep something *.php Returns list of .php files containing something
        let 12: (String, Vec<&str>) = ("", vec![]),
        //gf    Open file name under cursor
        let 13: (String, Vec<&str>) = ("", vec![]),
    }

    pub struct interface_commands_tabs_windows() {
        // :tabnew      Opens a new tab
        let 0: (String, Vec<&str>) = ("", vec![]),
        //gt    Go to next tab
        let 1: (String, Vec<&str>) = ("", vec![]),
        //:tabfirst     Go to first tab
        let 2: (String, Vec<&str>) = ("", vec![]),
        //:tablast      Go to last tab
        let 3: (String, Vec<&str>) = ("", vec![]),
        //:tabm n(position)     Rearrange open tabs
        let 4: (String, Vec<&str>) = ("", vec![]),
        //:tabdo %s/foo/bar/g   Execute same command in all tabs
        let 5: (String, Vec<&str>) = ("", vec![]),
        //:tab ball     Puts all open files in different tabs
        let 6: (String, Vec<&str>) = ("", vec![]),
        //:new myfile.txt       Edit myfile.txt in new window
        let 7: (String, Vec<&str>) = ("", vec![]),
        //:e filename   Edit filename in current window
        let 8: (String, Vec<&str>) = ("", vec![]),
        //:split myfile Split the window and open myfile
        let 9: (String, Vec<&str>) = ("", vec![]),
        //ctrl-w + Up arrow   Puts cursor in top window
        let 10: (String, Vec<&str>) = ("", vec![]),
        //ctrl-w ctrl-w (twice) Puts cursor in next window
        let 11: (String, Vec<&str>) = ("", vec![]), 
        //ctrl-w_       Maximize current window vertically
        let 12: (String, Vec<&str>) = ("", vec![]),
        //ctrl-w|       Maximize current window horizontally
        let 13: (String, Vec<&str>) = ("", vec![]),
        //ctrl-w=       Make all windows of the same size
        let 14: (String, Vec<&str>) = ("", vec![]), 
        //100 ctrl-w+   Add 100 lines to file in current window
        let 15: (String, Vec<&str>) = ("", vec![]),
        //:vsplit file  Split windows vertically
        let 16: (String, Vec<&str>) = ("", vec![]),
        //:sview file   Split windows vertically (read-only)
        let 17: (String, Vec<&str>) = ("", vec![]),
        //:hide Close current window
        let 18: (String, Vec<&str>) = ("", vec![]),
        //:­nly Close all windows, except the current
        let 19: (String, Vec<&str>) = ("", vec![]),
        //:b 4  Open tab #4 in current window
        let 20: (String, Vec<&str>) = ("", vec![]),

    }

    pub struct text_alignment_and_indentation() {
        // :set autoindent      Turn on auto-indentation
        let 0: (String, Vec<&str>) = ("", vec![]),
        // :%!fmt        Align all the lines
        let 1: (String, Vec<&str>) = ("", vec![]),
        // !}fmt Align all lines at the current position
        let 2: (String, Vec<&str>) = ("", vec![]),
        // 2!!fmt        Align the next two lines
        let 3: (String, Vec<&str>) = ("", vec![]),
        // :set smartindent      Turn on smart auto-indentation
        let 4: (String, Vec<&str>) = ("", vec![]), 
        // :set shiftwidth=8     Defines 8 spaces as indent size
        let 5: (String, Vec<&str>) = ("", vec![]),
        // ctrl-t, ctrl-d        Indent and un-indent in Insert Mode
        let 6: (String, Vec<&str>) = ("", vec![]), 
        // >>    Indent the current line
        let 7: (String, Vec<&str>) = ("", vec![]), 
        // <<    Un-indent the current line
        let 8: (String, Vec<&str>) = ("", vec![]),
        // =%    Indent the code between parenthesis
        let 9: (String, Vec<&str>) = ("", vec![]),
        // 1GVG= Indent the entire file
        let 10: (String, Vec<&str>) = ("", vec![]),

    }

    pub struct autocomplete_vim_commands() {
        // Ctrl+n Ctrl+p        Complete the suggested word (Insert Mode)
        let 0: (String, Vec<&str>) = ("", vec![]),
        // Ctrl+x Ctrl+l        Complete the suggested line
        let 1: (String, Vec<&str>) = ("", vec![]),
        // :set dictionary=en   Define en as active dictionary
        let 2: (String, Vec<&str>) = ("", vec![]),
        // Ctrl+x Ctrl+k        Complete with the active dictionary
        let 3: (String, Vec<&str>) = ("", vec![]),

    }

    pub struct unix_only_vim_commands() {
         // :!get        Execute the get Unix command, then return to Vim
         let 0: (String, Vec<&str>) = ("", vec![]),
         // !!get Execute the get Unix command and insert output in current file
         let 1: (String, Vec<&str>) = ("", vec![]),
         //:sh   Return to Unix shell
         let 2: (String, Vec<&str>) = ("", vec![]), 
         //$exit Exit the Unix shell and return to Vim
         let 3: (String, Vec<&str>) = ("", vec![]),
         // m {q-p}      Marks the current position as {q-p}
         let 4: (String, Vec<&str>) = ("", vec![]),
         //‘ {q-p}       Move to position {q-p} (used after marking)
         let 5: (String, Vec<&str>) = ("", vec![]),
         //“     Move to previously marked position
         let 6: (String, Vec<&str>) = ("", vec![]), 
         //:ab mail hi@designbombs.com   Define mail as abbreviation of hi@designbombs.com
         let 7: (String, Vec<&str>) = ("", vec![]),
    }
}

    
