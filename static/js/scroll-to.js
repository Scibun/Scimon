const ScrollTo = ( e => {

    let headerScrollToBtn = 'scrollToTopBtn';

    let top = e => {
        window.scroll({
            top: 0,
            left: 0,
            behavior: 'smooth'
        });
    };

    let element = (el) => {
        let getElement = document.getElementById(el);

        if (getElement) {
            getElement.scrollIntoView({ behavior: 'smooth' });
            Effects.bounce(getElement);
        }
    };

    let checkScroll = e => {
        const scrollToTopBtn = document.getElementById(headerScrollToBtn);
        
        if (window.scrollY > 0) {
            scrollToTopBtn.style.display = 'block';
        } else {
            scrollToTopBtn.style.display = 'none';
        }
    };
    
    return {
        top: () => { return top(); },
        element: (el) => { return element(el); },
        checkScroll: () => { return checkScroll(); },
    };

})();
