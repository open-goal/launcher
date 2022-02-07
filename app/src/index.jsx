import React, { Suspense } from "react";
import ReactDOM from "react-dom";
import i18n from "I18n/i18n.config";
import { I18nextProvider } from "react-i18next";
import Root from "Core/root";
import store, { history } from "Redux/store/store";
import "bulma/css/bulma.css";
import { ChakraProvider } from '@chakra-ui/react';

ReactDOM.render(
  <I18nextProvider i18n={i18n}>
    <Suspense fallback="loading">
      <ChakraProvider>
        <Root store={store} history={history}></Root>
      </ChakraProvider>
    </Suspense>
  </I18nextProvider>,
  document.getElementById("target")
);
