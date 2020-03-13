FROM ruby:buster
RUN apt update -y && apt upgrade -y && apt install -y git nodejs
RUN gem install bundler
RUN mkdir slate build
RUN git clone https://github.com/slatedocs/slate slate_orig/
RUN cd slate_orig/ && bundle install
CMD cd slate_orig && cp -r /slate/* source/ && bundle exec middleman build --clean && cp -r build/* /build/