use crate :: IconProps ; # [inline (never)] pub fn simple_icons_answer (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M5.486 0c-1.92 0-2.881 0-3.615.373A3.428 3.428 0 0 0 .373 1.871C-.001 2.605 0 3.566 0 5.486v9.6c0 1.92 0 2.88.373 3.613.329.645.853 1.17 1.498 1.498.734.374 1.695.375 3.615.375h11.657V24l.793-.396c2.201-1.101 3.3-1.652 4.105-2.473a6.852 6.852 0 0 0 1.584-2.56C24 17.483 24 16.251 24 13.79V5.486c0-1.92 0-2.881-.373-3.615A3.428 3.428 0 0 0 22.129.373C21.395-.001 20.434 0 18.514 0H5.486zm1.371 10.285h10.286a5.142 5.142 0 0 1-10.286.024v-.024z" /></ svg > } }