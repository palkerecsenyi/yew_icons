use crate :: IconProps ; # [inline (never)] pub fn feather_git_pull_request (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < circle cx = "18" cy = "18" r = "3" /> < circle cx = "6" cy = "6" r = "3" /> < path d = "M13 6h3a2 2 0 0 1 2 2v7" /> < line x1 = "6" y1 = "9" x2 = "6" y2 = "21" /> </ svg > } }