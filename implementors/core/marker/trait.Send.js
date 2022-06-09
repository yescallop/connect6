(function() {var implementors = {};
implementors["connect6"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"connect6/board/enum.Stone.html\" title=\"enum connect6::board::Stone\">Stone</a>","synthetic":true,"types":["connect6::board::Stone"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"connect6/board/enum.Axis.html\" title=\"enum connect6::board::Axis\">Axis</a>","synthetic":true,"types":["connect6::board::Axis"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/board/struct.Point.html\" title=\"struct connect6::board::Point\">Point</a>","synthetic":true,"types":["connect6::board::Point"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/board/struct.ParsePointError.html\" title=\"struct connect6::board::ParsePointError\">ParsePointError</a>","synthetic":true,"types":["connect6::board::ParsePointError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/board/struct.Slot.html\" title=\"struct connect6::board::Slot\">Slot</a>","synthetic":true,"types":["connect6::board::Slot"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/message/struct.Settings.html\" title=\"struct connect6::message::Settings\">Settings</a>","synthetic":true,"types":["connect6::message::Settings"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/message/struct.GameResult.html\" title=\"struct connect6::message::GameResult\">GameResult</a>","synthetic":true,"types":["connect6::message::GameResult"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"connect6/message/enum.GameResultKind.html\" title=\"enum connect6::message::GameResultKind\">GameResultKind</a>","synthetic":true,"types":["connect6::message::GameResultKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"connect6/message/enum.Event.html\" title=\"enum connect6::message::Event\">Event</a>","synthetic":true,"types":["connect6::message::Event"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/message/struct.FullEvent.html\" title=\"struct connect6::message::FullEvent\">FullEvent</a>","synthetic":true,"types":["connect6::message::FullEvent"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"connect6/message/enum.CmdError.html\" title=\"enum connect6::message::CmdError\">CmdError</a>","synthetic":true,"types":["connect6::message::CmdError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"connect6/message/enum.Cmd.html\" title=\"enum connect6::message::Cmd\">Cmd</a>","synthetic":true,"types":["connect6::message::Cmd"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/message/struct.FullCmd.html\" title=\"struct connect6::message::FullCmd\">FullCmd</a>","synthetic":true,"types":["connect6::message::FullCmd"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/channel/struct.CmdSender.html\" title=\"struct connect6::channel::CmdSender\">CmdSender</a>","synthetic":true,"types":["connect6::channel::CmdSender"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/player/struct.Console.html\" title=\"struct connect6::player::Console\">Console</a>","synthetic":true,"types":["connect6::player::Console"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/player/struct.Void.html\" title=\"struct connect6::player::Void\">Void</a>","synthetic":true,"types":["connect6::player::Void"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/player/struct.Chaos.html\" title=\"struct connect6::player::Chaos\">Chaos</a>","synthetic":true,"types":["connect6::player::Chaos"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/algorithm/struct.BitBoard.html\" title=\"struct connect6::algorithm::BitBoard\">BitBoard</a>","synthetic":true,"types":["connect6::algorithm::BitBoard"]},{"text":"impl&lt;const T:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/algorithm/struct.MctsState.html\" title=\"struct connect6::algorithm::MctsState\">MctsState</a>&lt;T&gt;","synthetic":true,"types":["connect6::algorithm::MctsState"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/struct.Builder.html\" title=\"struct connect6::Builder\">Builder</a>","synthetic":true,"types":["connect6::Builder"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/struct.Handle.html\" title=\"struct connect6::Handle\">Handle</a>","synthetic":true,"types":["connect6::Handle"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/struct.Control.html\" title=\"struct connect6::Control\">Control</a>","synthetic":true,"types":["connect6::Control"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"connect6/board/struct.Board.html\" title=\"struct connect6::board::Board\">Board</a>","synthetic":false,"types":["connect6::board::Board"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()