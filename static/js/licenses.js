const Licenses = ( e => {

    let classActivedElementRoot = 'actived';
    let nameElementRoot = Elements.licensesBox;
    let headerScrollToBtn = Elements.toggleLicensesBox;

    let keywords = [
        'GPL', 
        'BSD License', 
        'MIT License', 
        'Apache License', 
        'Creative Commons',
        'Boost Software License', 
        'GNU GENERAL PUBLIC LICENSE', 
        'GNU AFFERO GENERAL PUBLIC LICENSE', 
        'Mozilla Public License Version 2.0', 
    ];

    let list = e => {
        let foundLicenses = [];
        let existsLicense = false;
        let pageText = document.body.innerText;

        keywords.forEach( keyword => {
            if (pageText.includes(keyword)) {
                existsLicense = true;
                foundLicenses.push(keyword);
            }
        });

        return {
            exists: existsLicense,
            licenses: foundLicenses,
        };
    };

    let init = e => {
        let getLicenses = list();

        if (getLicenses.exists) {
            getLicenses.licenses.forEach( license => {
                let a = document.createElement('a');
                let idLicense = getLicenses.licenses.indexOf(license);

                a.textContent = license;
                a.onclick = e => { ScrollTo.license(idLicense); };

                document.getElementById(nameElementRoot).appendChild(a);
            });

            document.getElementById(headerScrollToBtn).style.display = 'inline';
        } else {
            document.getElementById(headerScrollToBtn).style.display = 'none';
        }
    };

    let forceHideBox = e => {
        let element = document.getElementById(nameElementRoot);

        element.style.display = 'none';
        toggleActivedClassLicensesBtn();
    };

    let formatBibText = text => {
        let textWithoutPTag = text.replace(
            /<\/?p>/g, ''
        );

        let braceIndex = textWithoutPTag.indexOf(
            '{', textWithoutPTag.indexOf('@')
        );

        let firstPart = textWithoutPTag.slice(
            0, braceIndex + 1
        );

        let secondPart = textWithoutPTag.slice(
            braceIndex + 1
        ).replace(
            /,/g, ',\n      '
        );

        return firstPart + secondPart.slice(0, -1) + '\n' + secondPart.slice(-1);
    };

    let toggleLicensesBox = e => {
        let element = document.getElementById(nameElementRoot);
        let elementBtn = document.getElementById(headerScrollToBtn);

        Animate.effect(element, 'animate__slideInDown');

        setTimeout( e => {
            if (element.style.display == 'block') {
                element.style.display = 'none';
                elementBtn.classList.remove(classActivedElementRoot);
            } else {
                element.style.display = 'block';
                elementBtn.classList.add(classActivedElementRoot);
            }
        }, 100)
    };

    let toggleActivedClassLicensesBtn = e => {
        let element = document.getElementById(nameElementRoot);
        let elementBtn = document.getElementById(headerScrollToBtn);
        let isActived = elementBtn.classList.contains(classActivedElementRoot);
        
        if (isActived || element.style.display != 'none') {
            elementBtn.classList.remove(classActivedElementRoot);
        } else {
            elementBtn.classList.add(classActivedElementRoot);
        }
    };
    
    return {
        element: nameElementRoot,
        toggleBtn: headerScrollToBtn,

        list: () => { return list(); },
        init: () => { return init(); },
        forceHideBox: () => { return forceHideBox(); },
        formatBibText: (text) => {return formatBibText(text); },
        toggleLicensesBox: () => { return toggleLicensesBox(); },
    };

})();
