var searchIndex = JSON.parse('{\
"connect6":{"doc":"A library for hosting Connect6 games asynchronously.","t":[3,3,3,0,11,11,11,11,11,11,11,11,0,11,11,12,0,12,11,12,11,11,11,11,11,11,11,0,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,13,4,13,3,13,13,3,3,3,4,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,3,2,2,11,11,11,11,11,11,11,11,11,11,11,11,11,5,5,13,13,13,4,4,13,13,13,13,13,4,3,3,13,3,4,13,13,13,13,13,13,3,13,13,13,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,3,3,8,3,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Builder","Control","Handle","board","board_size","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build","channel","clone","clone_into","cmd_tx","console","ctrl","default","event_rx","fmt","from","from","from","into","into","into","message","new","player","start","start","start_silent","subscribe","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","Ascending","Axis","Black","Board","Descending","Horizontal","ParsePointError","Point","Slot","Stone","Vertical","White","adjacent","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","contains_point","drop","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from_str","get","get_mut","hash","index","index_mut","into","into","into","into","into","into","is_empty","is_full","is_occupied","is_win_at","make_move","move_index","ne","new","new","opposite","set_stone","size","stone","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","unit_vec","x","y","CmdSender","Receiver","Sender","accept_or_offer_draw","borrow","borrow_mut","drop","from","into","into_full","make_move","split","stone","try_from","try_into","type_id","log","read_move","AcceptOrOfferDraw","BoardFull","BothPass","Cmd","CmdError","Disconnect","Disconnected","DrawOffer","DrawOfferAccepted","Error","Event","FullCmd","FullEvent","GameEnd","GameResult","GameResultKind","IllTimed","Move","Move","Occupied","OutOfBoard","RowCompleted","Settings","Settings","Timeout","Turn","board_size","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","cmd","eq","eq","event","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","into","into","into","into","into","into","into","into","kind","ne","stone","stone","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","winning_stone","0","0","0","0","0","0","0","Chaos","Console","Player","Void","attach","attach","attach","attach","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","from","from","from","into","into","into","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id"],"q":["connect6","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","connect6::board","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","connect6::channel","","","","","","","","","","","","","","","","connect6::console","","connect6::message","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","connect6::message::Cmd","connect6::message::CmdError","","connect6::message::Event","","","","connect6::player","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Builder for a game.","A game control.","A game handle.","Connect6 boards.","Sets the board size.","","","","","","","Builds the game handle.","Channel types for message passing between tasks.","","","The command sender.","Module for console logging and input.","The game control.","","The global event receiver.","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Message types that may be passed between tasks.","Creates a new builder with default values.","Player trait and implementations.","Starts a game with the given players, logging the events …","Starts the game and returns the result when the game is …","Starts a game with the given players silently.","Subscribes two notification event receivers (Black, White) …","","","","","","","","","","","The ascending diagonal axis, with a unit vector of <code>(1, 1)</code>.","Axes on the board.","A black stone.","A Connect6 board.","The descending diagonal axis, with a unit vector of <code>(1, -1)</code>…","The horizontal axis, with a unit vector of <code>(1, 0)</code>.","An error which can be returned when parsing a point.","A 2D point with <code>u32</code> coordinates.","A slot (namely intersection) on the board.","A stone on the board, either black or white.","The vertical axis, with a unit vector of <code>(0, 1)</code>.","A white stone.","Returns the adjacent point in the direction of the axis.","","","","","","","","","","","","","","","","","","","","","Returns <code>true</code> if the board contains a point.","","","","","","","","","","","Formats a <code>Point</code> as a point reference.","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Parses a point reference into a <code>Point</code>.","Returns a reference to a slot, or <code>None</code> if the point is out …","Returns a mutable reference to a slot, or <code>None</code> if the …","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns <code>true</code> if the slot is empty.","Returns <code>true</code> if the board is full.","Returns <code>true</code> if the slot is occupied.","Returns <code>true</code> if there is a six or overline of the given …","Makes a move on the board.","Returns the current move index starting from <code>1</code>, or <code>0</code> if …","","Creates a new <code>Board</code> with the given size.","Creates a new <code>Point</code> with the given coordinates.","Returns the opposite stone.","Sets the stone in the slot.","Returns the size of the board.","Returns the stone in the slot, or <code>None</code> if the slot is …","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the unit vector in the direction of the axis.","The horizontal coordinate.","The vertical coordinate.","A command sender.","","","Accepts or offers a draw.","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Consumes this <code>CmdSender</code> and returns the underlying full …","Makes a move.","Splits this anonymous sender into stone-specific (Black, …","Returns the stone of this sender, or <code>None</code> if this sender …","","","","Logs the game events in the console.","Reads a move from the console.","Accepts or offers a draw.","The board is full.","Both players passed.","A command sent from the player task.","Errors occurred by an invalid command.","Disconnects when the sender is dropped.","Player or server disconnected.","A draw is offered.","A draw offer has been accepted.","Error occurred by the last command.","An event sent from the game task.","A full command sent from the player task.","A full event sent from the game task.","Game ended.","The result of a game.","The reason for the end of a game.","Ill-timed command.","Move made.","A move.","The slot at the point is occupied.","The point is out of board.","A row has been completed.","The settings of a game.","Game settings.","Timeout.","Your turn.","The board size.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The command.","","","The event.","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","The kind of the result.","","The stone the event is associated with.","The stone that sent the command, or <code>None</code> if sent …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The winning stone, or <code>None</code> for a draw.","","","","","","","","A player that makes totally randomized moves.","A player that inputs moves from the console.","A trait for Connect6 players.","A player that passes on every move.","Attaches the player to the game.","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","",""],"i":[0,0,0,0,1,2,3,1,2,3,1,1,0,1,1,2,0,2,1,2,1,2,3,1,2,3,1,0,1,0,2,3,2,3,1,2,3,1,2,3,1,2,3,1,4,0,5,0,4,4,0,0,0,0,4,5,6,7,5,4,6,8,9,7,5,4,6,8,9,5,4,6,9,5,4,6,9,7,7,5,4,6,6,8,7,5,5,4,6,6,8,8,9,7,5,4,6,6,8,9,6,7,7,6,7,7,7,5,4,6,8,9,9,7,9,7,7,7,6,7,6,5,9,7,9,5,4,6,9,7,5,6,8,7,5,4,6,8,9,7,5,4,6,8,9,7,5,4,6,8,9,4,6,6,0,0,0,10,10,10,10,10,10,10,10,10,10,10,10,10,0,0,11,12,12,0,0,11,12,13,12,13,0,0,0,13,0,0,14,13,11,14,14,12,0,13,12,13,15,15,16,12,13,17,14,11,18,15,16,12,13,17,14,11,18,15,16,12,13,17,14,11,18,15,16,12,13,17,14,11,18,18,16,12,17,15,16,12,12,13,17,14,14,11,18,15,16,12,13,17,14,11,18,15,16,12,13,17,14,11,18,16,16,17,18,15,16,12,13,17,14,11,18,12,14,15,16,12,13,17,14,11,18,15,16,12,13,17,14,11,18,15,16,12,13,17,14,11,18,16,19,20,21,22,23,24,25,0,0,0,0,26,27,28,29,27,28,29,27,28,29,27,28,29,27,28,29,27,28,29,27,28,29,27,28,29],"f":[null,null,null,null,[[["",0],["u32",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["handle",3]],null,[[["",0]],["builder",3]],[[["",0],["",0]]],null,null,null,[[]],null,[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[]],null,[[]],[[["box",3]]],[[]],[[["",0]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,[[["axis",4],["bool",0]],["point",3]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["stone",4]],[[["",0]],["axis",4]],[[["",0]],["point",3]],[[["",0]],["slot",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["point",3]],["bool",0]],[[["",0]]],[[["",0],["stone",4]],["bool",0]],[[["",0],["axis",4]],["bool",0]],[[["",0],["point",3]],["bool",0]],[[["",0]],["bool",0]],[[["",0],["parsepointerror",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",0]],["result",4]],[[["",0],["point",3]],["option",4,[["slot",3]]]],[[["",0],["point",3]],["option",4,[["slot",3]]]],[[["",0],["",0]]],[[["",0]],["slot",3]],[[["",0]],["slot",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[["",0]],["bool",0]],[[["",0]],["bool",0]],[[["",0]],["bool",0]],[[["",0],["point",3],["stone",4]],["bool",0]],[[["",0],["stone",4]]],[[["",0]],["u32",0]],[[["",0],["point",3]],["bool",0]],[[["u32",0]],["board",3]],[[["u32",0],["u32",0]],["point",3]],[[],["stone",4]],[[["",0],["option",4,[["stone",4]]]]],[[["",0]],["u32",0]],[[["",0]],["option",4,[["stone",4]]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[]],null,null,null,null,null,[[["",0]]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]]],[[]],[[]],[[],["sender",3,[["fullcmd",3]]]],[[["",0],["option",4]]],[[]],[[["",0]],["option",4,[["stone",4]]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["receiver",3,[["fullevent",3]]]]],[[["stone",4]]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["settings",3]],[[["",0]],["gameresult",3]],[[["",0]],["gameresultkind",4]],[[["",0]],["event",4]],[[["",0]],["fullevent",3]],[[["",0]],["cmderror",4]],[[["",0]],["cmd",4]],[[["",0]],["fullcmd",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],null,[[["",0],["gameresult",3]],["bool",0]],[[["",0],["gameresultkind",4]],["bool",0]],null,[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["",0],["gameresult",3]],["bool",0]],null,null,[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,[[["receiver",3,[["event",4]]],["cmdsender",3]],["pin",3,[["box",3,[["future",8]]]]]],[[["receiver",3,[["event",4]]],["cmdsender",3]],["pin",3,[["box",3,[["future",8]]]]]],[[["receiver",3,[["event",4]]],["cmdsender",3]],["pin",3,[["box",3,[["future",8]]]]]],[[["receiver",3,[["event",4]]],["cmdsender",3]],["pin",3,[["box",3,[["future",8]]]]]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]]],"p":[[3,"Builder"],[3,"Handle"],[3,"Control"],[4,"Axis"],[4,"Stone"],[3,"Point"],[3,"Board"],[3,"ParsePointError"],[3,"Slot"],[3,"CmdSender"],[4,"Cmd"],[4,"GameResultKind"],[4,"Event"],[4,"CmdError"],[3,"Settings"],[3,"GameResult"],[3,"FullEvent"],[3,"FullCmd"],[13,"Move"],[13,"Occupied"],[13,"OutOfBoard"],[13,"Settings"],[13,"Move"],[13,"Error"],[13,"GameEnd"],[8,"Player"],[3,"Console"],[3,"Void"],[3,"Chaos"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
