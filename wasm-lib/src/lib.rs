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
         // open_myfile: String = ":e",
         0: ("Opens 'myfile' for editing".to_string(), ":e".to_string()),
         // :w - Save file
         // save_file: String = ":w",
         1: ("Save file".to_string(), ":w".to_string()),
         // :sav myfile.txt
         // save_myfile_txt: String = ":sav myfile.txt",
         
         // 😡 - Write changes to file and exit
         // ^ Not sure how to implement emojis rn, will deal with that later
         // :q! - Quit without saving changes
         quit_vim_nosave: String = ":q!",
         // :q - quit vim
         quit_vim: String = ":q",

    }

    pub struct find_and_replace() {
        // y    Copy the selected text
        copy_selected_text: String = "y",
        // p    Paste the selected text
        paste_selected_text: String = "p",
        // dd   Cut the current line
        cut_current_time: String = "dd",
        // yy   Copy the current line
        copy_current_line: String = "yy",
        // y$   Copy to EOL
        copy_to_eol: String = "y$",
        // D    Cut to EOL
        cut_to_eol: String = "D",

    }
    
    pub struct change_case_commands() {
        // Vu   Lowercase the entire line
        lowercase_entire_line: String = "Vu",

        //VU    Uppercase the entire line
        uppercase_entire_line: String = "VU",
        //g~~   Invert selected case
        invert_selected_line: String = "g~~",
        //vEU   Switch selected word to uppercase
        switch_word_to_uppercase: String = "vEU",
        //vE~   Modify the current word case
        modify_current_word_case: String = "vE~",
        // ggguG Set all text to lowercase
        set_all_text_lower: String = "ggguG",
        //gggUG Set all text to uppercase
        set_all_text_upper: String = "gggUG",
        //:set ignorecase       Ignore case when searching text
        set_ignorecase: String = ":set ignorecase",
        //:set smartcase        Ignore case in search, but not if an uppercase letter is used
        set_smartcase: String = ":set smartcase",
        //:%s/\<./\u&/g Sets the first letter of each word to uppercase
        first_letter_each_word_upper: String = ":%s/\<./\u&/g",
        //:%s/\<./\l&/g Sets the first letter of each word to lowercase
        first_letter_each_word_lower: String = ":%s/\<./\l&/g",
        //:%s/.*/\u&    Sets the first letter of each line to uppercase
        first_letter_each_line_uppercase: String =":%s/.*/\u&",
        //:%s/.*/\l&    Sets the first letter of each line to lowercase
        first_letter_each_line_lowercase: String = ":%s/.*/\l&",
    }

    pub struct navigate_within_file() {
        // k or Up Arrow        Move the cursor position up one line
        cursor_oneline_up: ("k", "[up arrow]"), 
        //j or Down Arrow Move the cursor down one line
        cursor_oneline_down: ("j", "[down arrow]"), 
        //e     Move the cursor to the end of the word
        move_cursor_word_end: "e",
        //b     Move the cursor to the beginning of the word
        move_cursor_word_begin: "b",
        //0     Move the cursor to the beginning of the line
        move_cursor_line_begin: "0",
        //G     Move the cursor to EOF
        move_cursor_EOF: "G",
        //gg    Move the cursor to the beginning of the file
        move_cursor_file_begin: "gg",
        //L     Move the cursor to the bottom of the screen
        move_cursor_bottom_screen: "L",
        //:80   Move the cursor to line number 80
        move_cursor_line_80: ":80",
        //%     Move the cursor to matching parenthesis
        move_cursor_matching_parenthesis: "%",
        //[[    Move the cursor to function start
        move_cursor_function_start: "[[",
        //[{    Move the cursor to block start
        move_cursor_function_start: "[{",

    }

    pub struct file_explorer_read_write() {
        // :1,10 w myfile       Saves lines 1 to 10 in myfile
        save_lines_1to10: ":1,10 w myfile",
        //:1,10 w >> myfile     Appends lines 1 to 10 to myfile
        append_lines_1to10: "1, 10w",
        //:r myfile     Inserts the content of myfile to current file
        insert_myfile_to_current: ":r myfile",
        //:23r myfile   Inserts the content of myfile under line 23
        insert_content_myfile_under_line_23: ":23r",
        //:e .  Open the File Explorer
        open_file_explorer: ":e .",
        //:Sex  Split window and open File Explorer
        split_window_file_explorer: ":Sex",
        //:Sex! Same as :Sex but splits window vertically
        split_window_vertically: ":Sex!",
        //:browse e     Graphical File Explorer
        graphical_file_explorer: ":browse e",
        //:ls   List buffers
        list_buffers: ":ls",
        //:cd ..        Move to parent or root directory
        move_to_parent_or_root_dir: ":cd",
        //:args List files
        list_files: ":args",
        //:args *.php   Open file list with .php extension
        open_file_list_with_php_extension: ":args *.php",
        //:grep something *.php Returns list of .php files containing something
        return_php_file_list_containing_something: ":grep something *.php",
        //gf    Open file name under cursor
        open_file_name_under_cursor: "gf",
    }

    pub struct interface_commands_tabs_windows() {
        // :tabnew      Opens a new tab
        open_new_tab: ":tabnew",
        //gt    Go to next tab
        go_to_next_tab: "gt",
        //:tabfirst     Go to first tab
        go_to_first_tab: ":tabfirst",
        //:tablast      Go to last tab
        go_to_last_tab: ":tablast",
        //:tabm n(position)     Rearrange open tabs
        rearrange_open_tabs: ":tabm n(position)",
        //:tabdo %s/foo/bar/g   Execute same command in all tabs
        execute_same_command_all_tabs: ":tabdo %s/foo/bar/g",
        //:tab ball     Puts all open files in different tabs
        all_open_tabs_to_diff: ":tab ball",
        //:new myfile.txt       Edit myfile.txt in new window
        edit_myfiletxt_in_new_window: ":new myfile.txt",
        //:e filename   Edit filename in current window
        edit_filename_in_current_window: ":e",
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

    
