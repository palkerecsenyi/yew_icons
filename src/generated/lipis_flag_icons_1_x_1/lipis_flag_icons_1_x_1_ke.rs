use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_ke (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } id = "flag-icons-ke" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < path id = "a" stroke - miterlimit = "10" d = "m-28.6 47.5 1.8 1 46.7-81c2.7-.6 4.2-3.2 5.7-5.8 1-1.8 5-8.7 6.7-17.7a58 58 0 0 0-11.9 14.7c-1.5 2.6-3 5.2-2.3 7.9z" /> </ defs > < path fill = "#fff" d = "M0 0h512v512H0z" /> < path d = "M0 0h512v153.6H0z" /> < path fill = "#060" d = "M0 358.4h512V512H0z" /> < g id = "b" transform = "matrix(3.2 0 0 3.2 255.8 256)" > < use href = "#a" width = "100%" height = "100%" stroke = "#000" /> < use href = "#a" width = "100%" height = "100%" fill = "#fff" /> </ g > < use href = "#b" width = "100%" height = "100%" transform = "matrix(-1 0 0 1 511.7 0)" /> < path fill = "#b00" d = "M255.8 102.4c-19.2 0-51.2 51.2-60.8 76.8H0v153.6h195c9.7 25.6 41.7 76.8 60.9 76.8 19.2 0 51.2-51.2 60.8-76.8H512V179.2H316.6c-9.6-25.6-41.6-76.8-60.8-76.8z" /> < path id = "c" d = "M316.6 332.8a220 220 0 0 0 16-76.8 220 220 0 0 0-16-76.8 220 220 0 0 0-16 76.8 220 220 0 0 0 16 76.8" /> < use href = "#c" width = "100%" height = "100%" transform = "matrix(-1 0 0 1 511.7 0)" /> < g fill = "#fff" transform = "matrix(3.2 0 0 3.2 255.8 256)" > < ellipse rx = "4" ry = "6" /> < path id = "d" d = "M1 5.8s4 8 4 21-4 21-4 21z" /> < use href = "#d" width = "100%" height = "100%" transform = "scale(-1)" /> < use href = "#d" width = "100%" height = "100%" transform = "scale(-1 1)" /> < use href = "#d" width = "100%" height = "100%" transform = "scale(1 -1)" /> </ g > </ svg > } }