const Effects = ( e => {

    let classesElementEffect = ['animate__animated', 'animate__bounce'];

    let bounce = el => {
        el.classList.remove(...classesElementEffect);
        setTimeout( e => { el.classList.add(...classesElementEffect); }, 100);
    };
    
    return {
        bounce: (el) => { return bounce(el); },
    };

})();
