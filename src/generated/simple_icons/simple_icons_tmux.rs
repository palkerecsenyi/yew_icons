use crate :: IconProps ; # [inline (never)] pub fn simple_icons_tmux (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M24 2.251V10.5H12.45V0h9.3A2.251 2.251 0 0 1 24 2.251zM12.45 11.4H24v10.5h-.008A2.25 2.25 0 0 1 21.75 24H2.25a2.247 2.247 0 0 1-2.242-2.1H0V2.251A2.251 2.251 0 0 1 2.25 0h9.3v21.6h.9V11.4zm11.242 10.5H.308a1.948 1.948 0 0 0 1.942 1.8h19.5a1.95 1.95 0 0 0 1.942-1.8z" /></ svg > } }