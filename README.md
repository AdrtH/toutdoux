# toutdoux
A todo gatherer

It is formatted to enable quick jump to file with Emacs (and probably other editor/IDE).  

# QUICK USE:  
It requires cargo to be up and running on your computer
```
cd <installation path>
cargo build
./toutdoux <path of file>
./toutdoux -r <path of folder> // if you want to 'grep' from a whole folder + subfolder
```

This has the advantage of organizing your todo by priority.  

# EXAMPLE
```
// TODO : change the string formatting for x  
// TODOOO :  will crash if y, should fix that   
// TODOO : this is very slow, consider changing the algorithm   
```  
Will display as  
```
priority:3 // TODOOO :  will crash if y, should fix that
priority:2 // TODOO : this is very slow, consider changing the algorithm
priority:1 // TODO : change the string formatting for x
```  
use toutdoux --help to view all parameters available

**WARNING** It only supports Unicode for the moment.  
**WARNING** If you use the recursive mode, I recommand putting a extension clause, (-e)  
**WARNING** It will always skip .o or no-extension file  
**WARNING** This is poorly written, it's my first time using Rust ever.  

This is not my idea, I saw this tool being used by some people and wanted to create it by myself, tailored to my likings,  
I unfortunately don't know where it originates, **be free to PR this readme to link the original work**.  
