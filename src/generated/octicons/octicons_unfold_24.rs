use crate :: IconProps ; # [inline (never)] pub fn octicons_unfold_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M12 23a.75.75 0 01-.53-.22l-3.25-3.25a.75.75 0 111.06-1.06L12 21.19l2.72-2.72a.75.75 0 111.06 1.06l-3.25 3.25A.75.75 0 0112 23z" />< path fill - rule = "evenodd" d = "M12 22.25a.75.75 0 01-.75-.75v-5.75a.75.75 0 011.5 0v5.75a.75.75 0 01-.75.75zM10.75 12a.75.75 0 01.75-.75h1a.75.75 0 110 1.5h-1a.75.75 0 01-.75-.75zm-8 0a.75.75 0 01.75-.75h1a.75.75 0 010 1.5h-1a.75.75 0 01-.75-.75zm12 0a.75.75 0 01.75-.75h1a.75.75 0 010 1.5h-1a.75.75 0 01-.75-.75zm-8 0a.75.75 0 01.75-.75h1a.75.75 0 010 1.5h-1a.75.75 0 01-.75-.75zm12 0a.75.75 0 01.75-.75h1a.75.75 0 010 1.5h-1a.75.75 0 01-.75-.75zM11.47 1.22a.75.75 0 011.06 0l3.25 3.25a.75.75 0 01-1.06 1.06L12 2.81 9.28 5.53a.75.75 0 01-1.06-1.06l3.25-3.25z" />< path fill - rule = "evenodd" d = "M12 1.5a.75.75 0 01.75.75v6a.75.75 0 01-1.5 0v-6A.75.75 0 0112 1.5z" /></ svg > } }