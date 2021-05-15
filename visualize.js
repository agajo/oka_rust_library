window.onload = (event)=>{
    e = document.getElementById("img_tag");
    i = 0;
    timer = setInterval(()=>{
        e.setAttribute("src","out_svg/"+("0000"+i).slice(-4)+".svg");
        i += 1;
    },100);
    e.onerror = ()=>{
        clearInterval(timer);
        i-=1;
        e.setAttribute("src","out_svg/"+("0000"+i).slice(-4)+".svg");
    };
}