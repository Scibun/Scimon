const Animate = ( e => {

    let getClasses = effect => {
        let getEffect = `animate__${ effect.replace('animate__', '') }`;
        return ['animate__animated', getEffect];
    };

    let makeEffect = (el, effect, timeOut = true) => {
        let classesEffect = getClasses(effect);
        el.classList.remove(...classesEffect);
        
        if (timeOut) {
            setTimeout( e => {
                el.classList.add(...classesEffect);
            }, 100);
        } else {
            el.classList.add(...classesEffect);
        }
    };
    
    return {
        effect: (el, effect, timeout = true) => {
            return makeEffect(
                el, 
                effect, 
                timeout
            );
        },
    };

})();
