use crate :: IconProps ; # [inline (never)] pub fn octicons_feed_trophy_16 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11 5h1v1.146a1 1 0 01-.629.928L11 7.223V5zM5 7.223l-.371-.149A1 1 0 014 6.146V5h1v2.223z" />< path fill - rule = "evenodd" d = "M8 16A8 8 0 108 0a8 8 0 000 16zM3 5a1 1 0 011-1h8a1 1 0 011 1v1.146a2 2 0 01-1.257 1.857l-.865.346a3.005 3.005 0 01-2.294 2.094C8.78 11.405 9.342 12 10.5 12a.5.5 0 010 1h-5a.5.5 0 010-1h.002c1.156 0 1.718-.596 1.914-1.557A3.005 3.005 0 015.122 8.35l-.865-.346A2 2 0 013 6.146V5z" /></ svg > } }