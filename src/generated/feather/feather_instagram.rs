use crate :: IconProps ; # [inline (never)] pub fn feather_instagram (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "2" y = "2" width = "20" height = "20" rx = "5" ry = "5" /> < path d = "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" /> < line x1 = "17.5" y1 = "6.5" x2 = "17.51" y2 = "6.5" /> </ svg > } }