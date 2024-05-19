const ScrollTo = ( e => {

    let headerScrollToBtn = 'scrollToTopBtn';
    let classNameLicensesItems = '.language-license';

    let top = e => {
        window.scroll({
            top: 0,
            left: 0,
            behavior: 'smooth'
        });
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
            Effects.bounce(getElement);
        }
    };

    let checkScroll = e => {
        let scrollToTopBtn = document.getElementById(headerScrollToBtn);
        
        if (window.scrollY > 0) {
            scrollToTopBtn.style.display = 'block';
        } else {
            scrollToTopBtn.style.display = 'none';
        }
    };

    let license = (id) => {
        let elements = document.querySelectorAll(classNameLicensesItems);

        if (elements.length > 0) {
            Licenses.forceHideBox();
            
            let parseId = parseInt(id);
            let pos = elements[parseId].getBoundingClientRect();

            window.scrollTo(0, window.scrollY + pos.top - 90);
            Effects.bounce(elements[parseId].parentElement);
        }
    };
    
    return {
        top: () => { return top(); },
        element: (el) => { return element(el); },
        license: (el) => { return license(el); },
        checkScroll: () => { return checkScroll(); },
    };

})();
