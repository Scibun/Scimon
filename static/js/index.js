window.onload = e => {

    Elements.pageTitle();

    MathDiagram.math();
    MathDiagram.diagram();

    Licenses.init();
    Citations.init();
    DocsSources.init();

    ScrollTo.checkScroll();
    window.addEventListener('scroll', ScrollTo.checkScroll);

    window.addEventListener("click", event => {
        if (event.target != document.getElementById(BibTex.modal)) {
            BibTex.close();
        }

        if (event.target != document.getElementById(Licenses.element)) {
            Licenses.forceHideBox();
        }
    });

    document.getElementById(BibTex.code).addEventListener('click', BibTex.copy);

    document.getElementById(ScrollTo.topBtn).addEventListener('click', ScrollTo.top);
    document.getElementById(Citations.toggleBtn).addEventListener('click', Citations.scrollTo);
    document.getElementById(DocsSources.toggleBtn).addEventListener('click', DocsSources.scrollTo);
    document.getElementById(Licenses.toggleBtn).addEventListener('click', Licenses.toggleLicensesBox);
    
};
