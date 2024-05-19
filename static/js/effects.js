const Effects = ( e => {

    let classRootAnimate = 'animate__animated';
    let classesBounceEffect = [classRootAnimate, 'animate__bounce'];

    let bounce = (el, timeOut = true) => {
        el.classList.remove(...classesBounceEffect);
        
        if (timeOut) {
            setTimeout( e => { el.classList.add(...classesBounceEffect); }, 100);
        } else {
            el.classList.add(...classesBounceEffect);
        }
    };
    
    return {
        bounce: (el) => { return bounce(el); },
    };

})();
