use crate :: IconProps ; # [inline (never)] pub fn octicons_globe_16 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M1.543 7.25h2.733c.144-2.074.866-3.756 1.58-4.948.12-.197.237-.381.353-.552a6.506 6.506 0 00-4.666 5.5zm2.733 1.5H1.543a6.506 6.506 0 004.666 5.5 11.13 11.13 0 01-.352-.552c-.715-1.192-1.437-2.874-1.581-4.948zm1.504 0h4.44a9.637 9.637 0 01-1.363 4.177c-.306.51-.612.919-.857 1.215a9.978 9.978 0 01-.857-1.215A9.637 9.637 0 015.78 8.75zm4.44-1.5H5.78a9.637 9.637 0 011.363-4.177c.306-.51.612-.919.857-1.215.245.296.55.705.857 1.215A9.638 9.638 0 0110.22 7.25zm1.504 1.5c-.144 2.074-.866 3.756-1.58 4.948-.12.197-.237.381-.353.552a6.506 6.506 0 004.666-5.5h-2.733zm2.733-1.5h-2.733c-.144-2.074-.866-3.756-1.58-4.948a11.738 11.738 0 00-.353-.552 6.506 6.506 0 014.666 5.5zM8 0a8 8 0 100 16A8 8 0 008 0z" /></ svg > } }