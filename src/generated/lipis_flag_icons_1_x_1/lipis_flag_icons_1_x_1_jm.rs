use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_jm (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-jm" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" > < path d = "m0 0 256 256L0 512zm512 0L256 256l256 256z" /> < path fill = "#090" d = "m0 0 256 256L512 0zm0 512 256-256 256 256z" /> < path fill = "#fc0" d = "M512 0h-47.7L0 464.3V512h47.7L512 47.7z" /> < path fill = "#fc0" d = "M0 0v47.7L464.3 512H512v-47.7L47.7 0z" /> </ g > </ svg > } }