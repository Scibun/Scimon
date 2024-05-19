window.onload = e => {

    Licenses.init();
    Citations.init();
    DocsSources.init();

    ScrollTo.checkScroll();
    window.addEventListener('scroll', ScrollTo.checkScroll);

    let pageTitle = document.title.split(':');
    document.getElementById('headerLabel').textContent = `${ pageTitle[1] }: ${ pageTitle[2] }`;
    
};
