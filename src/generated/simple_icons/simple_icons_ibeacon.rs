use crate :: IconProps ; # [inline (never)] pub fn simple_icons_ibeacon (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12 .053c-6.617 0-12 5.383-12 12 0 6.077 4.541 11.113 10.408 11.894v-.364C4.741 22.804.36 17.931.36 12.053.36 5.635 5.582.413 12 .413s11.64 5.222 11.64 11.64c0 5.878-4.38 10.751-10.048 11.53v.364C19.459 23.166 24 18.13 24 12.053c0-6.617-5.383-12-12-12zm0 1.696c-5.653 0-10.251 4.598-10.251 10.25 0 5.112 3.76 9.362 8.66 10.129v-.366c-4.7-.763-8.3-4.85-8.3-9.762 0-5.454 4.437-9.89 9.891-9.89s9.891 4.436 9.891 9.89c0 4.912-3.6 8.999-8.3 9.762v.366c4.9-.767 8.66-5.017 8.66-10.128 0-5.653-4.598-10.25-10.251-10.25zm0 1.736c-4.695 0-8.515 3.82-8.515 8.515 0 4.151 2.986 7.618 6.923 8.365v-.367C6.671 19.256 3.845 15.952 3.845 12c0-4.496 3.659-8.155 8.155-8.155 4.496 0 8.154 3.659 8.154 8.155 0 3.952-2.825 7.256-6.562 7.998v.367c3.937-.747 6.923-4.214 6.923-8.365 0-4.695-3.82-8.515-8.515-8.515zm0 1.725A6.798 6.798 0 0 0 5.21 12c0 3.196 2.22 5.883 5.198 6.602v-.372C7.63 17.52 5.57 14.996 5.57 12A6.437 6.437 0 0 1 12 5.57 6.437 6.437 0 0 1 18.43 12c0 2.996-2.06 5.52-4.838 6.23v.372c2.979-.719 5.198-3.406 5.198-6.602A6.798 6.798 0 0 0 12 5.21zm0 1.749A5.047 5.047 0 0 0 6.959 12a5.05 5.05 0 0 0 3.45 4.782v-.38A4.689 4.689 0 0 1 7.318 12c0-2.58 2.1-4.68 4.681-4.68s4.68 2.1 4.68 4.68a4.689 4.689 0 0 1-3.088 4.402v.38A5.05 5.05 0 0 0 17.042 12 5.047 5.047 0 0 0 12 6.96zm0 1.737A3.308 3.308 0 0 0 8.696 12c0 1.245.692 2.33 1.712 2.894v-.42a2.943 2.943 0 1 1 3.184 0v.42A3.306 3.306 0 0 0 15.304 12 3.308 3.308 0 0 0 12 8.696zm0 1.712A1.592 1.592 0 0 0 10.408 12 1.592 1.592 0 0 0 12 13.592 1.592 1.592 0 0 0 13.592 12 1.592 1.592 0 0 0 12 10.408Z" /></ svg > } }