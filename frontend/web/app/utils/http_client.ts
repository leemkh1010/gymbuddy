const call_api = async (url: string, method: string, body: string): Promise<Response> => {
  const option = {
    method: method,
    headers: {
      'Content-Type': 'application/json',
      'Authorization': '',
    },
  } as RequestInit;

  if (method !== 'GET') {
    option.body = body;
  }

  return await fetch(url, option);
};

function exponentialRetry(maxRetries: number = 5, initialDelay: number = 1000) {
  return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
      const originalMethod = descriptor.value;

      descriptor.value = async function (...args: string[]) {
          let attempt = 0;
          let delay = initialDelay;

          while (attempt < maxRetries) {
              try {
                  return await originalMethod.apply(this, args);
              } catch (error) {
                  attempt++;
                  if (attempt >= maxRetries) {
                      throw error;
                  }
                  console.log(`Attempt ${attempt} failed. Retrying in ${delay}ms...`);
                  await new Promise(resolve => setTimeout(resolve, delay));
                  delay *= 2; // Exponential backoff
              }
          }
      };

      return descriptor;
  };
}

export class HttpClient {
  // @exponentialRetry(5, 1000)
  public static async GET(url: string): Promise<Response> {
    return await call_api(url, 'GET', '');
  }

  public static async POST(url: string, body: string): Promise<Response> {
    return await call_api(url, 'POST', body);
  }

  public static async PUT(url: string, body: string): Promise<Response> {
    return await call_api(url, 'PUT', body);
  }

  public static async DELETE(url: string): Promise<Response> {
    return await call_api(url, 'DELETE', '');
  }
}