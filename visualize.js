window.onload = (event)=>{
    frame_input = document.getElementById("frame_input");
    fps_input = document.getElementById("fps_input");
    fps = 15;
    fps_input.value = fps;

    img_tag = document.getElementById("img_tag");
    i = 0;
    timer = setInterval(()=>{
        img_tag.setAttribute("src","out_svg/"+("0000"+i).slice(-4)+".svg");
        i += 1;
        frame_input.value=i;
    },1000/fps);
    img_tag.onerror = ()=>{
        clearInterval(timer);
        i-=1;
        frame_input.value=i;
        img_tag.setAttribute("src","out_svg/"+("0000"+i).slice(-4)+".svg");
    };

    pause_button = document.getElementById("pause_button");
    pause_button.onclick = ()=>{
        clearInterval(timer);
    }

    play_button = document.getElementById("play_button");
    play_button.onclick=()=>{
        timer = setInterval(()=>{
            img_tag.setAttribute("src","out_svg/"+("0000"+i).slice(-4)+".svg");
            i += 1;
            frame_input.value=i;
        },1000/fps);
    }

    frame_input.oninput=()=>{
        val = parseInt(frame_input.value);
        if(!isNaN(val)){
            i = val;
            img_tag.setAttribute("src","out_svg/"+("0000"+i).slice(-4)+".svg");
        }
    }

    fps_input.oninput=()=>{
        val = parseInt(fps_input.value);
        if(!isNaN(val)){
            fps = val;
        }
    }

    reset_button = document.getElementById("reset_button");
    reset_button.onclick=()=>{
        i=0;
        frame_input.value=i;
        img_tag.setAttribute("src","out_svg/"+("0000"+i).slice(-4)+".svg");
    }
}