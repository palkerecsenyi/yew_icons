use crate :: IconProps ; # [inline (never)] pub fn simple_icons_svgo (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M21.9685 9.5707a10.3841 10.3841 0 0 0-1.2419-2.851c.4116-.5796.8064-1.1723 1.1723-1.7999l-2.8271-2.8187c-.6432.3768-1.2527.7848-1.8491 1.2107a10.2604 10.2604 0 0 0-2.7599-1.1387c-.12-.72-.2676-1.4483-.4572-2.1731h-4.0054c-.1871.7104-.3275 1.4207-.4475 2.1311a10.302 10.302 0 0 0-2.845 1.1279c-.5724-.4068-1.1591-.798-1.7771-1.1579L2.1023 4.9282c.3408.582.7068 1.1375 1.0871 1.6799a10.3302 10.3302 0 0 0-1.3199 2.9878 25.065 25.065 0 0 0-1.8683.4032v4.0006c.5796.1512 1.1567.2736 1.7363.3792a10.327 10.327 0 0 0 1.2815 3.2602c-.3204.462-.6288.9372-.9168 1.4315l2.8283 2.8283c.48-.2784.9348-.5736 1.3811-.8808A10.3366 10.3366 0 0 0 9.6522 22.41c.0972.5304.2112 1.0607.348 1.5899h4.0006c.1416-.5436.2568-1.0859.36-1.6307a10.2988 10.2988 0 0 0 3.2566-1.4063c.4716.3264.96.642 1.4567.936l2.8271-2.8283c-.3144-.5364-.6528-1.0511-1.0043-1.5599a10.3406 10.3406 0 0 0 1.1999-3.109c.6336-.1116 1.2683-.24 1.9019-.4056v-3.997c-.6755-.1752-1.3534-.3132-2.0302-.4284zm-2.9147 5.1886c-.6873 1.9556-2.1612 3.5519-4.0522 4.399-1.8966.856-4.0703.8774-5.9901.0792-1.8862-.7889-3.4536-2.3813-4.1746-4.339-.7227-1.9308-.6184-4.0925.288-5.9445a7.6099 7.6099 0 0 1 3.8242-3.6466c1.9315-.8372 4.2099-.8069 6.1185.0816a7.5908 7.5908 0 0 1 3.643 3.5506c.8889 1.8067 1.0111 3.9209.3432 5.8197zm-1.6883-.1416c.069.3397.0197.6928-.1396 1.0006-.3857.7451-1.3023 1.0365-2.0474.6508-.7451-.3857-1.0365-1.3023-.6508-2.0474l-1.1999-.8964a2.076 2.076 0 0 1-2.4179.4368l-.96 1.4951c.3821.3619.5043.9206.3083 1.409-.2639.6575-1.0107.9765-1.6682.7127-.6575-.2639-.9765-1.0107-.7127-1.6682.2639-.6575 1.0107-.9765 1.6682-.7127l.96-1.4963a2.0736 2.0736 0 0 1-.7008-2.1215l-2.0627-.8087a.9598.9598 0 0 1-.873.3903c-.5275-.0534-.9118-.5243-.8583-1.0518.0534-.5275.5243-.9117 1.0518-.8583.5275.0534.9118.5243.8583 1.0518v.0204l2.0603.81c.5027-.9713 1.6739-1.3843 2.6747-.9432l.9695-1.6643a1.0355 1.0355 0 0 1-.2804-1.0488c.1638-.548.7408-.8594 1.2887-.6956.548.1638.8594.7408.6956 1.2887-.1638.548-.7408.8594-1.2887.6956l-.9731 1.6643c.8459.633 1.0815 1.8004.5472 2.7119l1.1999.8928a1.5195 1.5195 0 0 1 .7612-.404c.8222-.167 1.6241.3642 1.7911 1.1864h-.0013z" /></ svg > } }