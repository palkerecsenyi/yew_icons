use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_gh (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-gh" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#006b3f" d = "M0 0h640v480H0z" /> < path fill = "#fcd116" d = "M0 0h640v320H0z" /> < path fill = "#ce1126" d = "M0 0h640v160H0z" /> < path d = "m320 160 52 160-136.1-98.9H404L268 320z" /> </ svg > } }