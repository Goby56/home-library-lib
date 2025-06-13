import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';

const QRBOT_SCAN_URL = "https://qrbot.net/x-callback-url/scan?x-success=";
const SELF_URL = "http://192.168.1.223:5173/scan";

export const load: PageLoad = async ({ url }) => {
  const shouldRedirect = url.searchParams.get("redirect");

  // qrbot reference
  // https://ioshelp.qrbot.net/xcallback
  
  if (shouldRedirect === "true") {
    redirect(302, QRBOT_SCAN_URL + SELF_URL);
  }

  const source = url.searchParams.get("x-source")
  const content = url.searchParams.get("content")
  const format = url.searchParams.get("format")
  
  if (source === "qrbot") {
    if (format === "ean13") {
        redirect(302, "/add?isbn=" + content)
    }
  }

  return {
      source: source,
      content: content,
      format: format,
  };
};
