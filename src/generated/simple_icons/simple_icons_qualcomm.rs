use crate :: IconProps ; # [inline (never)] pub fn simple_icons_qualcomm (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12 0C6.22933 0 1.5761 4.48645 1.5761 10.47394c0 6.00417 4.65323 10.47394 10.4239 10.47394.98402 0 1.93468-.13343 2.8353-.3836l1.13412 2.9187c.11675.31688.35025.51702.7672.51702h1.80125c.43364 0 .75052-.28353.55038-.83391l-1.46768-3.81932c2.88534-1.81793 4.80333-5.03683 4.80333-8.8895C22.4239 4.48644 17.77067 0 12 0m4.53648 16.5615l-1.31758-3.41904c-.11675-.28353-.35024-.55038-.85059-.55038h-1.71786c-.43363 0-.7672.28353-.56706.83391l1.73454 4.48645c-.56706.1501-1.18416.21682-1.81793.21682-4.2196 0-7.22168-3.31897-7.22168-7.65532C4.77832 6.1376 7.7804 2.81862 12 2.81862s7.22168 3.31898 7.22168 7.65532c0 2.5351-1.01737 4.70327-2.6852 6.08756" /></ svg > } }