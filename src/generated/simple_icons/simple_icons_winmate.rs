use crate :: IconProps ; # [inline (never)] pub fn simple_icons_winmate (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M5.785 4.058l-4.473.004L1.311.01 19.469 0c3.514.42 3.199 4.047 3.199 4.047l-2.156-.002-2.769 15.888L14.79 4.049l-4.731.005.856 7.376-2.137 8.507L5.785 4.058zM4.491 21.373L1.317 8.52l.009 12.338C1.756 23.983 4.629 24 4.629 24l1.687-.001c-1.393-.69-1.825-2.626-1.825-2.626zm9.237.659l-1.724-6.724-1.673 6.678c-.517 1.652-1.702 2.009-1.702 2.009l6.602-.002c-1.206-.499-1.503-1.961-1.503-1.961zm8.949-17.643l-2.844 15.865c-.711 3.767-2.285 3.738-2.285 3.738l5.141-.008-.012-19.595z" /></ svg > } }