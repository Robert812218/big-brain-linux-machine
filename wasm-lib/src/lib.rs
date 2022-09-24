use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
pub fn add_test() {
    assert_eq!(5 + 5, add(5, 5));
}

/////////
//#[wasm-bindgen]
//pub mod cat {
//    pub fn meow() {
//        crate::dog::woof();
//    }
//}
//
//#[wasm-bindgen]
//pub mod dog {
//    pub fn woof() {
//        println!("Woof Woof!");
//    }
//}
//
//#[wasm-bindgen]
//pub fn test_one() {
//    cat::meow();
//}
////////////

#[wasm_bindgen]
pub mod command_list() {
    pub struct save_and_exit {
         // :e myfile - Opens "myfile" for editing
         open_myfile: String = ":e",
         // :w - Save file
         save_file: String = ":w",
         // :sav myfile.txt
         save_myfile_txt: String = ":sav myfile.txt",
         // 😡 - Write changes to file and exit
         // ^ Not sure how to implement emojis rn, will deal with that later
         // :q! - Quit without saving changes
         quit_vim_nosave: String = ":q!",
         // :q - quit vim
         quit_vim: String = ":q",

    }

    pub struct find_and_replace() {
        // y    Copy the selected text
        copy_selected_text: String = "y";
        // p    Paste the selected text
        paste_selected_text: String = "p";
        // dd   Cut the current line
        cut_current_time: String = "dd";
        // yy   Copy the current line
        copy_current_line: String = "yy";
        // y$   Copy to EOL
        copy_to_eol: String = "y$";
        // D    Cut to EOL
        cut_to_eol: String = "D";

    }
    
    pub struct change_case_commands() {
        // Vu   Lowercase the entire line
        lowercase_entire_line: string = "Vu";
        //VU    Uppercase the entire line
        uppercase_entire_line: String = "VU";
        //g~~   Invert selected case
        invert_selected_line: String = "g~~";
        //vEU   Switch selected word to uppercase
        switch_word_to_uppercase: String = "vEU";
        //vE~   Modify the current word case
        modify_current_word_case: String = "vE~";
        // ggguG Set all text to lowercase
        set_all_text_lower: String = "ggguG";
        //gggUG Set all text to uppercase
        set_all_text_upper: String = "gggUG";
        //:set ignorecase       Ignore case when searching text
        set_ignorecase: String = ":set ignorecase";
        //:set smartcase        Ignore case in search, but not if an uppercase letter is used
        set_smartcase: String = ":set smartcase";
        //:%s/\<./\u&/g Sets the first letter of each word to uppercase
        first_letter_each_word_upper: String = ":%s/\<./\u&/g";
        //:%s/\<./\l&/g Sets the first letter of each word to lowercase
        first_letter_each_word_lower: String = "";
        //:%s/.*/\u&    Sets the first letter of each line to uppercase
        first_letter_each_line_uppercase: String ="";
        //:%s/.*/\l&    Sets the first letter of each line to lowercase
        first_letter_each_line_lowercase: String = "";
    }

    pub struct navigate_within_file() {
        // k or Up Arrow        Move the cursor position up one line
        //j or Down Arrow       Move the cursor down one line
        //e     Move the cursor to the end of the word
        //b     Move the cursor to the beginning of the word
        //0     Move the cursor to the beginning of the line
        //G     Move the cursor to EOF
        //gg    Move the cursor to the beginning of the file
        //L     Move the cursor to the bottom of the screen
        //:80   Move the cursor to line number 80
        //%     Move the cursor to matching parenthesis
        //[[    Move the cursor to function start
        //[{    Move the cursor to block start

    }

    pub struct file_explorer_read_write() {
        // :1,10 w myfile       Saves lines 1 to 10 in myfile
        //:1,10 w >> myfile     Appends lines 1 to 10 to myfile
        //:r myfile     Inserts the content of myfile to current file
        //:23r myfile   Inserts the content of myfile under line 23
        //:e .  Open the File Explorer
        //:Sex  Split window and open File Explorer
        //:Sex! Same as :Sex but splits window vertically
        //:browse e     Graphical File Explorer
        //:ls   List buffers
        //:cd ..        Move to parent or root directory
        //:args List files
        //:args *.php   Open file list with .php extension
        //:grep something *.php Returns list of .php files containing something
        //gf    Open file name under cursor

    }

    pub struct interface_commands_tabs_windows() {
        // :tabnew      Opens a new tab
        //gt    Go to next tab
        //:tabfirst     Go to first tab
        //:tablast      Go to last tab
        //:tabm n(position)     Rearrange open tabs
        //:tabdo %s/foo/bar/g   Execute same command in all tabs
        //:tab ball     Puts all open files in different tabs
        //:new myfile.txt       Edit myfile.txt in new window
        //:e filename   Edit filename in current window
        //:split myfile Split the window and open myfile
        //ctrl-w + Up arrow     Puts cursor in top window
        //ctrl-w ctrl-w (twice) Puts cursor in next window
        //ctrl-w_       Maximize current window vertically
        //ctrl-w|       Maximize current window horizontally
        //ctrl-w=       Make all windows of the same size
        //100 ctrl-w+   Add 100 lines to file in current window
        //:vsplit file  Split windows vertically
        //:sview file   Split windows vertically (read-only)
        //:hide Close current window
        //:­nly Close all windows, except the current
        //:b 4  Open tab #4 in current window

    }

    pub struct text_alignment_and_indentation() {
        // :set autoindent      Turn on auto-indentation
        // :%!fmt        Align all the lines
        // !}fmt Align all lines at the current position
        // 2!!fmt        Align the next two lines
        // :set smartindent      Turn on smart auto-indentation
        // :set shiftwidth=8     Defines 8 spaces as indent size
        // ctrl-t, ctrl-d        Indent and un-indent in Insert Mode
        // >>    Indent the current line
        // <<    Un-indent the current line
        // =%    Indent the code between parenthesis
        // 1GVG= Indent the entire file

    }

    pub struct autocomplete_vim_commands() {
        // Ctrl+n Ctrl+p        Complete the suggested word (Insert Mode)
        // Ctrl+x Ctrl+l        Complete the suggested line
        // :set dictionary=en   Define en as active dictionary
        // Ctrl+x Ctrl+k        Complete with the active dictionary

    }

    pub struct unix_only_vim_commands() {
         // :!get        Execute the get Unix command, then return to Vim
         // !!get Execute the get Unix command and insert output in current file
         //:sh   Return to Unix shell
         //$exit Exit the Unix shell and return to Vim
         //
         // m {q-p}      Marks the current position as {q-p}
         //‘ {q-p}       Move to position {q-p} (used after marking)
         //“     Move to previously marked position
         //:ab mail hi@designbombs.com   Define mail as abbreviation of hi@designbombs.com
    }
}

        
    
