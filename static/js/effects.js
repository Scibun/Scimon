const Effects = ( e => {

    let classRootAnimate = 'animate__animated';
    let classesBounceEffect = [classRootAnimate, 'animate__bounce'];
    let classesHeartBeatEffect = [classRootAnimate, 'animate__heartBeat'];

    let bounce = (el, timeOut = true) => {
        el.classList.remove(...classesBounceEffect);
        
        if (timeOut) {
            setTimeout( e => {
                el.classList.add(...classesBounceEffect);
            }, 100);
        } else {
            el.classList.add(...classesBounceEffect);
        }
    };

    let heartBeat = (el, timeOut = true) => {
        el.classList.remove(...classesHeartBeatEffect);
        
        if (timeOut) {
            setTimeout( e => {
                el.classList.add(...classesHeartBeatEffect);
            }, 100);
        } else {
            el.classList.add(...classesHeartBeatEffect);
        }
    };
    
    return {
        bounce: (el) => { return bounce(el); },
        heartBeat: (el) => { return heartBeat(el); },
    };

})();
