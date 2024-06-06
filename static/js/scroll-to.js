const ScrollTo = ( e => {

    let headerScrollToBtn = Elements.scrollToTopBtn;

    let top = e => {
        window.scroll({
            top: 0,
            left: 0,
            behavior: 'smooth'
        });
    };

    let checkScroll = e => {
        let scrollToTopBtn = document.getElementById(headerScrollToBtn);
        
        if (window.scrollY > 0) {
            scrollToTopBtn.style.display = 'block';
        } else {
            scrollToTopBtn.style.display = 'none';
        }
    };

    let element = (el, type = 'id') => {
        let getElement;
        
        if (type == 'id') {
            getElement = document.getElementById(el);
        } else {
            getElement = document.getElementsByClassName(el);
        }

        if (getElement) {
            getElement.scrollIntoView({ behavior: 'smooth' });

            Animate.effect(getElement, 'animate__pulse');
        }
    };
    
    return {
        topBtn: headerScrollToBtn,

        top: () => { return top(); },
        element: (el) => { return element(el); },
        checkScroll: () => { return checkScroll(); },
    };

})();
