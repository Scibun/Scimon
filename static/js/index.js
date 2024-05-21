window.onload = e => {

    Elements.pageTitle();

    Licenses.init();
    Citations.init();
    MathDiagram.init();
    DocsSources.init();

    ScrollTo.checkScroll();
    window.addEventListener('scroll', ScrollTo.checkScroll);

    window.addEventListener("click", event => {
        BibTex.autoHideClickOffset(event);
        Licenses.autoHideClickOffset(event);
    });

    document.getElementById(BibTex.code).addEventListener('click', BibTex.copy);

    document.getElementById(ScrollTo.topBtn).addEventListener('click', ScrollTo.top);
    document.getElementById(Citations.toggleBtn).addEventListener('click', Citations.scrollTo);
    document.getElementById(DocsSources.toggleBtn).addEventListener('click', DocsSources.scrollTo);
    document.getElementById(Licenses.toggleBtn).addEventListener('click', Licenses.toggleLicensesBox);
    
};
